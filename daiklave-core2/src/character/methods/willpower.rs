use crate::{Character, willpower::Willpower, CharacterMutationError};

impl<'source> Character<'source> {
    /// Returns the character's current willpower amount and permanent rating.
    pub fn willpower(&self) -> &Willpower {
        &self.willpower
    }

    /// Checks if the character's current willpower can be set to the specified
    /// amount.
    pub fn check_set_current_willpower(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets the character's willpower to the specified amount. This is allowed
    /// to exceed their ordinary rating.
    pub fn set_current_willpower(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.current = amount;
        Ok(self)
    }

    /// Checks if the character's permanent willpower can be set to the
    /// specified dot level.
    pub fn check_set_willpower_rating(&self, _dots: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets the character's permanent willpower rating to the specified dots
    /// amount. This will also reset their current willpower amount to be the
    /// same amount.
    pub fn set_willpower_rating(&mut self, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.rating = dots;
        self.willpower.current = dots;
        Ok(self)
    }
}