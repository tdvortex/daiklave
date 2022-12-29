use serde::{Deserialize, Serialize};

use crate::{
    Character, CharacterMutationError, CharacterView, Solar, solar::SolarView,
};

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

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        *self = ExaltState::Mortal;
        Ok(self)
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltStateView<'source> {
    Mortal,
    Exalted(ExaltTypeView<'source>),
}

impl<'source> Default for ExaltStateView<'source> {
    fn default() -> Self {
        Self::Mortal
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal)
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        *self = ExaltStateView::Mortal;
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltType {
    Solar(Solar),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltTypeView<'source> {
    Solar(SolarView<'source>),
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

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
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

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }
}
