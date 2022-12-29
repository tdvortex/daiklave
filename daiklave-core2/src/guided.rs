use thiserror::Error;

use crate::{id::CharacterId, CharacterMutation, CharacterMutationError, CharacterView, AttributeName};

pub fn begin_guided_builder(id: CharacterId) -> GuidedCharacterEventSource {
    GuidedCharacterEventSource {
        history: vec![GuidedCharacterMutation::CharacterMutation(CharacterMutation::SetId(id))],
        future: Vec::new(),
    }
}

pub enum GuidedCharacterMutation {
    CharacterMutation(CharacterMutation),
    SetStage(GuidedStage),
    SetExaltation(ExaltationChoice),
}

pub struct GuidedCharacterEventSource {
    history: Vec<GuidedCharacterMutation>,
    future: Vec<GuidedCharacterMutation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    ChooseNameAndConcept,
    ChooseExaltation,
    ChooseAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExaltationChoice {
    Mortal,
}

#[derive(Debug, Error)]
pub enum GuidedCharacterError {
    #[error("An error in applying the mutation to the base character")]
    CharacterMutationError(#[from] CharacterMutationError),
    #[error("An error in trying to move stages in the wrong order")]
    StageOrderError,
    #[error("An error trying to move because previous stage is not complete")]
    StageIncompleteError,
}

impl GuidedCharacterEventSource {
    fn mortal_bonus_points_remaining(&self) -> i32 {
        let mut bonus_points = 21;

        let character_view = self.as_character_view().expect("History should be valid");

        // Attribute bonus points costs
        let physical_attributes = character_view.attributes().dots(AttributeName::Strength) + character_view.attributes().dots(AttributeName::Dexterity) + character_view.attributes().dots(AttributeName::Stamina);
        let mental_attributes = character_view.attributes().dots(AttributeName::Perception) + character_view.attributes().dots(AttributeName::Intelligence) + character_view.attributes().dots(AttributeName::Wits);
        let social_attributes = character_view.attributes().dots(AttributeName::Charisma) + character_view.attributes().dots(AttributeName::Manipulation) + character_view.attributes().dots(AttributeName::Appearance);
    
        let primary = physical_attributes.max(mental_attributes).max(social_attributes) - 3;
        let tertiary = physical_attributes.min(mental_attributes).min(social_attributes) - 3;
        let secondary = physical_attributes + mental_attributes + social_attributes - primary - tertiary - 9;

        let attributes_cost = (primary - primary.min(6) + secondary - secondary.min(4)) * 4 + (tertiary - tertiary.min(3)) * 3;

        bonus_points -= attributes_cost as i32;
        bonus_points
    }

    pub fn bonus_points_remaining(&self) -> i32 {
        let maybe_exaltation_choice = self.exaltation_choice();
        if let Some(exaltation_choice) = maybe_exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => self.mortal_bonus_points_remaining()
            }
        } else {
            0
        }
    }

    fn current_stage(&self) -> GuidedStage {
        self.history.iter().filter_map(|gcm| if let GuidedCharacterMutation::SetStage(stage) = gcm {
            Some(stage)
        } else {
            None
        }).fold(GuidedStage::ChooseNameAndConcept, |_, stage| *stage)
    }

    fn exaltation_choice(&self) -> Option<ExaltationChoice> {
        self.history.iter().filter_map(|gcm| if let GuidedCharacterMutation::SetExaltation(exaltation_choice) = gcm {
            Some(*exaltation_choice)
        } else {
            None
        }).next()
    }

    fn as_character_view(&self) -> Result<CharacterView, GuidedCharacterError> {
        self.history
        .iter()
        .filter_map(|gcm| if let GuidedCharacterMutation::CharacterMutation(cm) = gcm {
            Some(cm)
        } else {
            None
        })
        .fold(Ok(CharacterView::default()), |res, mutation| {
            res.and_then(|mut view| {
                view.apply_mutation(mutation)?;
                Ok(view)
            })
        })
        .map_err(|e| GuidedCharacterError::CharacterMutationError(e))
    }

    fn check_character_mutation(&self, mutation: &CharacterMutation) -> Result<(), GuidedCharacterError> {
        let character_view = self.as_character_view()?;

        match mutation {
            other => {
                character_view.check_mutation(other).map_err(|e| GuidedCharacterError::CharacterMutationError(e))
            }
        }
    }

    fn validate_stage_complete(&self) -> Result<(), GuidedCharacterError> {
        let _character_view = self.as_character_view()?;
        match self.current_stage() {
            GuidedStage::ChooseNameAndConcept => {
                if self.history.iter().find(|gcm| if let GuidedCharacterMutation::CharacterMutation(CharacterMutation::SetName(_)) = gcm {
                    true
                } else {
                    false
                }).is_some() {
                    Ok(())
                } else {
                    Err(GuidedCharacterError::StageIncompleteError)
                }
            }
            GuidedStage::ChooseExaltation => {
                if self.history.iter().find(|gcm| if let GuidedCharacterMutation::SetExaltation(_) = gcm {
                    true
                } else {
                    false
                }).is_some() {
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
            | (GuidedStage::ChooseExaltation, GuidedStage::ChooseAttributes) => self.validate_stage_complete(),
            _ => Err(GuidedCharacterError::StageOrderError),
        }
    }

    pub fn check_mutation(&self, mutation: &GuidedCharacterMutation) -> Result<(), GuidedCharacterError> {
        match mutation {
            GuidedCharacterMutation::CharacterMutation(character_mutation) => self.check_character_mutation(character_mutation),
            GuidedCharacterMutation::SetStage(stage) => self.check_stage_advance(*stage),
            GuidedCharacterMutation::SetExaltation(exaltation) => {
                if let GuidedStage::ChooseExaltation = self.current_stage() {
                    Ok(())
                } else {
                    Err(GuidedCharacterError::StageOrderError)
                }
            }
        }
    }

    pub fn apply_mutation(&mut self, mutation: GuidedCharacterMutation) -> Result<&mut Self, GuidedCharacterError> {
        self.check_mutation(&mutation)?;
        self.future = Vec::new();
        self.history.push(mutation);

        Ok(self)
    }

    pub fn can_undo(&self) -> bool {
        self.history.len() > 1 // Don't undo SetId 
    }

    pub fn undo(&mut self) -> bool {
        if self.can_undo() {
            self.future.push(self.history.pop().unwrap());
            true
        } else {
            false
        }
    }

    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    pub fn redo(&mut self) -> bool {
        if self.can_redo() {
            self.history.push(self.future.pop().unwrap());
            true
        } else {
            false
        }
    }
}