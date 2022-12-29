use thiserror::Error;

use crate::{
    id::CharacterId, AttributeName, CharacterMutation, CharacterMutationError, CharacterView,
};

/// Initiates a new guided character builder.
pub fn begin_guided_builder(id: CharacterId) -> GuidedEventSource {
    GuidedEventSource {
        history: vec![GuidedMutation::CharacterMutation(CharacterMutation::SetId(
            id,
        ))],
        future: Vec::new(),
    }
}

/// The operations you can do during a guided character building process.
pub enum GuidedMutation {
    /// Apply a standard character mutation (with additional validation).
    CharacterMutation(CharacterMutation),
    /// Move on to the next stage of the builder. Note that because different
    /// Exalt types have different stages, some stages may be skipped or done
    /// in a different order.
    SetStage(GuidedStage),
    /// Choose a specific Exalt type (or Mortal), without necessarily setting
    /// all exaltations up-front.
    SetExaltation(ExaltationChoice),
}

/// An event-sourced guided character builder, supporting undo/redo.
pub struct GuidedEventSource {
    history: Vec<GuidedMutation>,
    future: Vec<GuidedMutation>,
}

/// A view into the current state of the guided character builder, including
/// any partial or temporarily incomplete state.
pub struct GuidedView<'source> {
    character_view: CharacterView<'source>,
    stage: GuidedStage,
    bonus_points: i32,
    exaltation_choice: Option<ExaltationChoice>,
}

impl GuidedEventSource {
    /// Derives the current state of the partially-complete character,
    /// including all state which is character-creation-only (like bonus points)
    pub fn as_guided_view(&self) -> Result<GuidedView, GuidedError> {
        let mut guided_view = GuidedView {
            character_view: CharacterView::default(),
            stage: GuidedStage::ChooseNameAndConcept,
            bonus_points: 0,
            exaltation_choice: None,
        };

        // Don't use GuidedView::apply_mutation() to avoid redundant bonus
        // point recalculations
        for guided_mutation in self.history.iter() {
            match guided_mutation {
                GuidedMutation::CharacterMutation(character_mutation) => {
                    guided_view
                        .character_view
                        .apply_mutation(character_mutation)?;
                }
                GuidedMutation::SetStage(stage) => {
                    guided_view.stage = *stage;
                }
                GuidedMutation::SetExaltation(exaltation_choice) => {
                    guided_view.exaltation_choice = Some(*exaltation_choice);
                }
            }
        }
        guided_view.update_bonus_points();

        Ok(guided_view)
    }
}

impl<'source> GuidedView<'source> {
    fn attributes_buckets(&self) -> (u8, u8, u8) {
        let physical_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Strength)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Dexterity)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Stamina);
        let mental_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Perception)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Intelligence)
            + self.character_view.attributes().dots(AttributeName::Wits);
        let social_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Charisma)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Manipulation)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Appearance);

        let primary = physical_attributes
            .max(mental_attributes)
            .max(social_attributes)
            - 3;
        let tertiary = physical_attributes
            .min(mental_attributes)
            .min(social_attributes)
            - 3;
        let secondary =
            physical_attributes + mental_attributes + social_attributes - primary - tertiary - 9;

        (primary, secondary, tertiary)
    }

    fn mortal_attributes_bonus_points_spent(&self) -> i32 {
        let (primary, secondary, tertiary) = self.attributes_buckets();
        ((primary - primary.min(6) + secondary - secondary.min(4)) * 4
            + (tertiary - tertiary.min(3)) * 3)
            .into()
    }

    fn solar_attributes_bonus_points_spent(&self) -> i32 {
        let (primary, secondary, tertiary) = self.attributes_buckets();
        ((primary - primary.min(8) + secondary - secondary.min(6)) * 4
            + (tertiary - tertiary.min(4)) * 3)
            .into()
    }

    fn update_bonus_points(&mut self) {
        if let Some(exaltation_choice) = self.exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => {
                    self.bonus_points = 21;
                    self.bonus_points -= self.mortal_attributes_bonus_points_spent();
                }
                ExaltationChoice::Dawn
                | ExaltationChoice::Zenith
                | ExaltationChoice::Twilight
                | ExaltationChoice::Night
                | ExaltationChoice::Eclipse => {
                    self.bonus_points = 15;
                    self.bonus_points -= self.solar_attributes_bonus_points_spent();
                }
            }
        } else {
            self.bonus_points = 0;
        }
    }

    fn validate_stage_complete(&self) -> Result<(), GuidedError> {
        if !match self.stage {
            GuidedStage::ChooseNameAndConcept => true,
            GuidedStage::ChooseExaltation => self.exaltation_choice.is_some(),
            GuidedStage::ChooseAttributes => {
                if let Some(exaltation_choice) = self.exaltation_choice {
                    match exaltation_choice {
                        ExaltationChoice::Mortal => {
                            let (primary, secondary, tertiary) = self.attributes_buckets();
                            primary >= 6 && secondary >= 4 && tertiary >= 3
                        }
                        ExaltationChoice::Dawn
                        | ExaltationChoice::Zenith
                        | ExaltationChoice::Twilight
                        | ExaltationChoice::Night
                        | ExaltationChoice::Eclipse => {
                            let (primary, secondary, tertiary) = self.attributes_buckets();
                            primary >= 8 && secondary >= 6 && tertiary >= 4
                        }
                    }
                } else {
                    return Err(GuidedError::StageOrderError);
                }
            }
        } {
            Err(GuidedError::StageIncompleteError)
        } else {
            Ok(())
        }
    }

    /// The number of available Bonus Points to spend.
    pub fn bonus_points_remaining(&self) -> i32 {
        self.bonus_points
    }

    /// Applies a mutation to the character view. \n Note that unlike
    /// CharacterView::apply_mutation, this operation takes self and not &mut
    /// self. This is because a CharacterMutation may be valid for a
    /// CharacterView but invalid for a GuidedView; applying the mutation will
    /// leave the GuidedView in an invalid state that must be discarded.
    pub fn apply_mutation(
        mut self,
        guided_mutation: &'source GuidedMutation,
    ) -> Result<Self, GuidedError> {
        match guided_mutation {
            GuidedMutation::CharacterMutation(character_mutation) => {
                self.character_view
                    .apply_mutation(character_mutation)
                    .map_err(GuidedError::CharacterMutationError)?;
                self.update_bonus_points();
            }
            GuidedMutation::SetStage(stage) => {
                self.validate_stage_complete()?;

                match (self.stage, stage) {
                    (GuidedStage::ChooseNameAndConcept, GuidedStage::ChooseExaltation)
                    | (GuidedStage::ChooseExaltation, GuidedStage::ChooseAttributes) => Ok(()),
                    _ => Err(GuidedError::StageOrderError),
                }?;
            }
            GuidedMutation::SetExaltation(exaltation_choice) => {
                self.exaltation_choice = Some(*exaltation_choice);
                self.update_bonus_points();
            }
        }

        if self.bonus_points < 0 {
            return Err(GuidedError::InsufficientBonusPoints);
        }

        Ok(self)
    }
}

/// The different phases of a guided character builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    /// The first stage, choosing a character name and (optional) concept.
    ChooseNameAndConcept,
    /// The second stage, choosing the Exaltation for the character (or Mortal).
    ChooseExaltation,
    /// The attribute selection stage. Comes after ChooseExaltation for
    /// Mortals and Solars.
    ChooseAttributes,
}

/// The supported options for Exaltations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExaltationChoice {
    /// No exaltation, just a heroic mortal.
    Mortal,
    /// Dawn caste Solar.
    Dawn,
    /// Zenith caste Solar.
    Zenith,
    /// Twilight caste Solar.
    Twilight,
    /// Night caste Solar.
    Night,
    /// Eclipse caste Solar.
    Eclipse,
}

/// The possible errors occurring in the guided character builder.
#[derive(Debug, Error)]
pub enum GuidedError {
    /// An error in applying the mutation to the base character
    #[error("Could not apply mutation to base character")]
    CharacterMutationError(#[from] CharacterMutationError),
    /// An error in trying to move stages in the wrong order
    #[error("Cannot move stages out of order")]
    StageOrderError,
    /// An error trying to move because previous stage is not complete
    #[error("Cannot move to the next stage while previous is incomplete")]
    StageIncompleteError,
    /// An error in trying to spend more bonus points than are available
    #[error("Cannot spend more bonus points than are available")]
    InsufficientBonusPoints,
}

impl GuidedEventSource {
    /// Checks if a GuidedCharacterMutation can be successfully applied.
    pub fn check_mutation(&self, mutation: &GuidedMutation) -> Result<(), GuidedError> {
        self.as_guided_view()?.apply_mutation(mutation)?;
        Ok(())
    }

    /// Apply a mutation, inserting it into the event history. This will erase
    /// all previously undone operations.
    pub fn apply_mutation(&mut self, mutation: GuidedMutation) -> Result<&mut Self, GuidedError> {
        self.check_mutation(&mutation)?;
        self.future = Vec::new();
        self.history.push(mutation);

        Ok(self)
    }

    /// Returns true if there is an operation which can be undone.
    pub fn can_undo(&self) -> bool {
        self.history.len() > 1 // Don't undo SetId
    }

    /// Attempts to undo the previous operation, returns true if successful.
    pub fn undo(&mut self) -> bool {
        if self.can_undo() {
            self.future.push(self.history.pop().unwrap());
            true
        } else {
            false
        }
    }

    /// Returns true if there is an operation which can be redone.
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Attempts to redo the last undone operation, returns true if successful.
    pub fn redo(&mut self) -> bool {
        if self.can_redo() {
            self.history.push(self.future.pop().unwrap());
            true
        } else {
            false
        }
    }
}
