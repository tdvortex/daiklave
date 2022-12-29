use thiserror::Error;

use crate::{id::CharacterId, CharacterMutation, CharacterMutationError, CharacterView};

pub fn begin_guided_builder(id: CharacterId) -> GuidedCharacterEventSource {
    GuidedCharacterEventSource {
        history: vec![GuidedCharacterMutation::CharacterMutation(CharacterMutation::SetId(id))],
        future: Vec::new(),
    }
}

pub enum GuidedCharacterMutation {
    CharacterMutation(CharacterMutation),
    SetStage(GuidedStage),
}

pub struct GuidedCharacterEventSource {
    history: Vec<GuidedCharacterMutation>,
    future: Vec<GuidedCharacterMutation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    ChooseNameAndConcept,
    ChooseExaltation,
}

#[derive(Debug, Error)]
pub enum GuidedCharacterError {
    #[error("An error in applying the mutation to the base character")]
    CharacterMutationError(#[from] CharacterMutationError),
    #[error("An error in trying to move stages in the wrong order")]
    StageOrderError,
}

impl GuidedCharacterEventSource {
    fn current_stage(&self) -> GuidedStage {
        self.history.iter().filter_map(|gcm| if let GuidedCharacterMutation::SetStage(stage) = gcm {
            Some(stage)
        } else {
            None
        }).fold(GuidedStage::ChooseNameAndConcept, |_, stage| *stage)
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
            GuidedStage::ChooseNameAndConcept => Ok(()),
            GuidedStage::ChooseExaltation => Ok(()), // TODO
        }
    }

    fn check_stage_advance(&self, stage: GuidedStage) -> Result<(), GuidedCharacterError> {
        self.validate_stage_complete()?;

        match (self.current_stage(), stage) {
            (GuidedStage::ChooseNameAndConcept, GuidedStage::ChooseExaltation) => self.validate_stage_complete(),
            _ => Err(GuidedCharacterError::StageOrderError),
        }
    }

    pub fn check_mutation(&self, mutation: &GuidedCharacterMutation) -> Result<(), GuidedCharacterError> {
        match mutation {
            GuidedCharacterMutation::CharacterMutation(character_mutation) => self.check_character_mutation(character_mutation),
            GuidedCharacterMutation::SetStage(stage) => self.check_stage_advance(*stage),
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