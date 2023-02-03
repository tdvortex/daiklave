use std::num::NonZeroU16;

use crate::{
    exaltation::{
        exalt::{essence::EssenceError, exalt_type::ExaltType},
        Exaltation,
    },
    experience::Experience,
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Get the experience pools for the character.
    pub fn experience(&self) -> Experience {
        let base = self.experience;
        let exalt = match &self.exaltation {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => match &exalt.exalt_type {
                ExaltType::Solar(solar) => Some(solar.experience()),
            },
        };

        Experience { base, exalt }
    }

    /// Add base/normal experience points to the character.
    pub fn gain_base_experience(
        &mut self,
        amount: NonZeroU16,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.experience.gain(amount);
        Ok(self)
    }

    /// Spend base/normal experience points.
    pub fn spend_base_experience(
        &mut self,
        amount: NonZeroU16,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.experience.spend(amount)?;
        Ok(self)
    }

    /// Add experience points to the character's Exalt experience pool.
    pub fn gain_exalt_experience(
        &mut self,
        amount: NonZeroU16,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::EssenceError(EssenceError::Mortal));
            }
            Exaltation::Exalt(exalt) => match &mut exalt.exalt_type {
                ExaltType::Solar(solar) => {
                    solar.experience.gain(amount);
                }
            },
        };
        Ok(self)
    }

    /// Spend experience points from the character's Exalt experience pool.
    pub fn spend_exalt_experience(
        &mut self,
        amount: NonZeroU16,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::EssenceError(EssenceError::Mortal));
            }
            Exaltation::Exalt(exalt) => match &mut exalt.exalt_type {
                ExaltType::Solar(solar) => {
                    solar.experience.spend(amount)?;
                }
            },
        };
        Ok(self)
    }
}
