use thiserror::Error;

use crate::{
    id::CharacterId, AttributeName, CharacterMutation, CharacterMutationError, CharacterView,
};

/// Initiates a new guided character builder.
pub fn begin_guided_builder(id: CharacterId) -> GuidedCharacterEventSource {
    GuidedCharacterEventSource {
        history: vec![GuidedCharacterMutation::CharacterMutation(
            CharacterMutation::SetId(id),
        )],
        future: Vec::new(),
    }
}

/// The operations you can do during a guided character building process.
pub enum GuidedCharacterMutation {
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
pub struct GuidedCharacterEventSource {
    history: Vec<GuidedCharacterMutation>,
    future: Vec<GuidedCharacterMutation>,
}

/// The different phases of a guided character builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    /// The first stage, choosing a character name and (optional) concept.
    ChooseNameAndConcept,
    /// The second stage, choosing the Exaltation for the character (or Mortal).
    ChooseExaltation,
    /// The attribute selection stage. Comes after ChooseExaltation for
    /// Mortals.
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
pub enum GuidedCharacterError {
    /// An error in applying the mutation to the base character
    #[error("Could not apply mutation to base character")]
    CharacterMutationError(#[from] CharacterMutationError),
    /// An error in trying to move stages in the wrong order
    #[error("Cannot move stages out of order")]
    StageOrderError,
    /// An error trying to move because previous stage is not complete
    #[error("Cannot move to the next stage while previous is incomplete")]
    StageIncompleteError,
}

impl GuidedCharacterEventSource {
    fn mortal_bonus_points_remaining(&self) -> i32 {
        let mut bonus_points = 21;

        let character_view = self.as_character_view().expect("History should be valid");

        // Attribute bonus points costs
        let physical_attributes = character_view.attributes().dots(AttributeName::Strength)
            + character_view.attributes().dots(AttributeName::Dexterity)
            + character_view.attributes().dots(AttributeName::Stamina);
        let mental_attributes = character_view.attributes().dots(AttributeName::Perception)
            + character_view
                .attributes()
                .dots(AttributeName::Intelligence)
            + character_view.attributes().dots(AttributeName::Wits);
        let social_attributes = character_view.attributes().dots(AttributeName::Charisma)
            + character_view
                .attributes()
                .dots(AttributeName::Manipulation)
            + character_view.attributes().dots(AttributeName::Appearance);

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

        let attributes_cost = (primary - primary.min(6) + secondary - secondary.min(4)) * 4
            + (tertiary - tertiary.min(3)) * 3;

        bonus_points -= attributes_cost as i32;
        bonus_points
    }

    fn solar_bonus_points_remaining(&self) -> i32 {
        let mut bonus_points = 15;
        let character_view = self.as_character_view().expect("History should be valid");

        // Attribute bonus points costs
        let physical_attributes = character_view.attributes().dots(AttributeName::Strength)
            + character_view.attributes().dots(AttributeName::Dexterity)
            + character_view.attributes().dots(AttributeName::Stamina);
        let mental_attributes = character_view.attributes().dots(AttributeName::Perception)
            + character_view
                .attributes()
                .dots(AttributeName::Intelligence)
            + character_view.attributes().dots(AttributeName::Wits);
        let social_attributes = character_view.attributes().dots(AttributeName::Charisma)
            + character_view
                .attributes()
                .dots(AttributeName::Manipulation)
            + character_view.attributes().dots(AttributeName::Appearance);

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

        let attributes_cost = (primary - primary.min(8) + secondary - secondary.min(6)) * 4
            + (tertiary - tertiary.min(4)) * 3;

        bonus_points -= attributes_cost as i32;
        bonus_points
    }

    /// The number of character creation Bonus Points remaining. Returns 0
    /// before ExaltationChoice is selected.
    pub fn bonus_points_remaining(&self) -> i32 {
        let maybe_exaltation_choice = self.exaltation_choice();
        if let Some(exaltation_choice) = maybe_exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => self.mortal_bonus_points_remaining(),
                ExaltationChoice::Dawn
                | ExaltationChoice::Zenith
                | ExaltationChoice::Twilight
                | ExaltationChoice::Night
                | ExaltationChoice::Eclipse => self.solar_bonus_points_remaining(),
            }
        } else {
            0
        }
    }

    fn current_stage(&self) -> GuidedStage {
        self.history
            .iter()
            .filter_map(|gcm| {
                if let GuidedCharacterMutation::SetStage(stage) = gcm {
                    Some(stage)
                } else {
                    None
                }
            })
            .fold(GuidedStage::ChooseNameAndConcept, |_, stage| *stage)
    }

    fn exaltation_choice(&self) -> Option<ExaltationChoice> {
        self.history
            .iter()
            .filter_map(|gcm| {
                if let GuidedCharacterMutation::SetExaltation(exaltation_choice) = gcm {
                    Some(*exaltation_choice)
                } else {
                    None
                }
            })
            .next()
    }

    fn as_character_view(&self) -> Result<CharacterView, GuidedCharacterError> {
        self.history
            .iter()
            .filter_map(|gcm| {
                if let GuidedCharacterMutation::CharacterMutation(cm) = gcm {
                    Some(cm)
                } else {
                    None
                }
            })
            .fold(Ok(CharacterView::default()), |res, mutation| {
                res.and_then(|mut view| {
                    view.apply_mutation(mutation)?;
                    Ok(view)
                })
            })
            .map_err(GuidedCharacterError::CharacterMutationError)
    }

    fn check_character_mutation(
        &self,
        mutation: &CharacterMutation,
    ) -> Result<(), GuidedCharacterError> {
        let character_view = self.as_character_view()?;
        character_view
            .check_mutation(mutation)
            .map_err(GuidedCharacterError::CharacterMutationError)
    }

    fn validate_stage_complete(&self) -> Result<(), GuidedCharacterError> {
        let _character_view = self.as_character_view()?;
        match self.current_stage() {
            GuidedStage::ChooseNameAndConcept => {
                if self.history.iter().any(|gcm| {
                    matches!(
                        gcm,
                        GuidedCharacterMutation::CharacterMutation(CharacterMutation::SetName(_),)
                    )
                }) {
                    Ok(())
                } else {
                    Err(GuidedCharacterError::StageIncompleteError)
                }
            }
            GuidedStage::ChooseExaltation => {
                if self
                    .history
                    .iter()
                    .any(|gcm| matches!(gcm, GuidedCharacterMutation::SetExaltation(_)))
                {
                    Ok(())
                } else {
                    Err(GuidedCharacterError::StageIncompleteError)
                }
            }
            GuidedStage::ChooseAttributes => Ok(()), // TODO
        }
    }

    fn check_stage_advance(&self, stage: GuidedStage) -> Result<(), GuidedCharacterError> {
        self.validate_stage_complete()?;

        match (self.current_stage(), stage) {
            (GuidedStage::ChooseNameAndConcept, GuidedStage::ChooseExaltation)
            | (GuidedStage::ChooseExaltation, GuidedStage::ChooseAttributes) => {
                self.validate_stage_complete()
            }
            _ => Err(GuidedCharacterError::StageOrderError),
        }
    }

    /// Checks if a GuidedCharacterMutation can be successfully applied.
    pub fn check_mutation(
        &self,
        mutation: &GuidedCharacterMutation,
    ) -> Result<(), GuidedCharacterError> {
        match mutation {
            GuidedCharacterMutation::CharacterMutation(character_mutation) => {
                self.check_character_mutation(character_mutation)
            }
            GuidedCharacterMutation::SetStage(stage) => self.check_stage_advance(*stage),
            GuidedCharacterMutation::SetExaltation(_) => {
                if let GuidedStage::ChooseExaltation = self.current_stage() {
                    Ok(())
                } else {
                    Err(GuidedCharacterError::StageOrderError)
                }
            }
        }
    }

    /// Apply a mutation, inserting it into the event history. This will erase
    /// all previously undone operations.
    pub fn apply_mutation(
        &mut self,
        mutation: GuidedCharacterMutation,
    ) -> Result<&mut Self, GuidedCharacterError> {
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
