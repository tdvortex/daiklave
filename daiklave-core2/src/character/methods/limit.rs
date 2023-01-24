use std::num::NonZeroU8;

use crate::{Character, exaltation::{exalt::{Limit, essence::EssenceError}, Exaltation}, CharacterMutationError};

impl<'source> Character<'source> {
    /// If the character has Limit, returns its details.
    pub fn limit(&self) -> Option<Limit<'source>> {
        match &self.exaltation {
            Exaltation::Mortal(_) => todo!(),
            Exaltation::Exalt(exalt) => {
                exalt.exalt_type.limit()
            }
        }
    }

    /// Adds Limit to the Exalt's Limit track, to a maximum of 10.
    pub fn gain_limit(&mut self, amount: NonZeroU8) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            Exaltation::Mortal(_) => Err(CharacterMutationError::EssenceError(EssenceError::Mortal)),
            Exaltation::Exalt(exalt) => {
                exalt.exalt_type.limit_mut().ok_or(CharacterMutationError::EssenceError(EssenceError::NoLimit))?.gain_limit(amount);
                Ok(self)
            }
        }
    }

    /// Removes Limit from the Exalt's Limit track, to a minimum of 0.
    pub fn reduce_limit(&mut self, amount: NonZeroU8) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            Exaltation::Mortal(_) => Err(CharacterMutationError::EssenceError(EssenceError::Mortal)),
            Exaltation::Exalt(exalt) => {
                exalt.exalt_type.limit_mut().ok_or(CharacterMutationError::EssenceError(EssenceError::NoLimit))?.remove_limit(amount);
                Ok(self)
            }
        }
    }

    /// Sets the Exalt's limit trigger.
    pub fn set_limit_trigger(&mut self, trigger: &'source str) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            Exaltation::Mortal(_) => Err(CharacterMutationError::EssenceError(EssenceError::Mortal)),
            Exaltation::Exalt(exalt) => {
                exalt.exalt_type.limit_mut().ok_or(CharacterMutationError::EssenceError(EssenceError::NoLimit))?.set_trigger(trigger);
                Ok(self)
            }
        }
    }
}
    