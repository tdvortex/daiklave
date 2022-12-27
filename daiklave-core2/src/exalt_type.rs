use serde::{Deserialize, Serialize};

use crate::{Character, CharacterMutationError, CharacterView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltState {
    Mortal,
    Exalted(ExaltType),
}

impl Default for ExaltState {
    fn default() -> Self {
        Self::Mortal
    }
}

impl ExaltState {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal)
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn is_solar(&self) -> bool {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        *self = ExaltState::Mortal;
        Ok(self)
    }

    pub fn check_set_solar(
        &self,
        _solar_traits: &SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar_traits: &SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self = Self::Exalted(ExaltType::Solar(solar_traits.clone()));
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltType {
    Solar(SolarTraits),
}

impl ExaltType {
    pub fn is_solar(&self) -> bool {
        true
    }
}

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits;

pub struct SolarTraitsBuilder;

impl SolarTraits {
    /// Creates a builder to construct SolarTraits.
    pub fn builder() -> SolarTraitsBuilder {
        SolarTraitsBuilder
    }
}

impl SolarTraitsBuilder {
    /// Consumes the builder to finalize Solar Traits.
    pub fn build(self) -> SolarTraits {
        SolarTraits
    }
}

impl Character {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(
        &self,
        solar_traits: &SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// De-Exalts character, setting them to be mortal.
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_mortal()?;
        Ok(self)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation.
    pub fn set_solar(
        &mut self,
        solar_traits: &SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }
}

impl<'source> CharacterView<'source> {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(
        &self,
        solar_traits: &SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// De-Exalts character, setting them to be mortal.
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_mortal()?;
        Ok(self)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation.
    pub fn set_solar(
        &mut self,
        solar_traits: &SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }
}
