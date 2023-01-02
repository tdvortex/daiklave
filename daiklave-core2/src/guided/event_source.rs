use crate::Character;

use super::{
    error::GuidedError, guided_mutation::GuidedMutation, guided_stage::GuidedStage,
    guided_state::GuidedState,
};

/// An event-sourced guided character builder, supporting undo/redo.
#[derive(Debug, Default)]
pub struct GuidedEventSource {
    history: Vec<GuidedMutation>,
    future: Vec<GuidedMutation>,
}

impl GuidedEventSource {
    /// Derives the current state of the partially-complete character,
    /// including all state which is character-creation-only (like bonus points)
    pub fn as_guided_view(&self) -> Result<GuidedState, GuidedError> {
        let mut guided_view = GuidedState {
            character_view: Character::default(),
            stage: GuidedStage::ChooseNameAndConcept,
            bonus_points: 0,
            merit_dots: 0,
            exaltation_choice: None,
            solar_caste_abilities: None,
            solar_supernal_ability: None,
            solar_favored_abilities: None,
            martial_arts_styles: None,
            sorcery_archetype: None,
            shaping_ritual: None,
            control_spell: None,
        };

        for guided_mutation in self.history.iter() {
            guided_view = guided_view.apply_mutation(guided_mutation)?;
        }

        Ok(guided_view)
    }

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
