use std::num::NonZeroU8;

use crate::{willpower::Willpower, Character, CharacterMutationError};

impl<'source> Character<'source> {
    /// Returns the character's current willpower amount and permanent rating.
    pub fn willpower(&self) -> &Willpower {
        &self.willpower
    }

    /// Increases the character's current willpower by the specified amount.
    /// This may take the character's current total above their maximum.
    pub fn gain_willpower(
        &mut self,
        _amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Decreases the character's current willpower by the specified amount,
    /// to a minimum of 0.
    pub fn spend_willpower(
        &mut self,
        _amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        todo!()
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

    /// Sets the character's permanent willpower rating to the specified dots
    /// amount. This will also reset their current willpower amount to be the
    /// same amount.
    pub fn set_willpower_rating(
        &mut self,
        dots: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.rating = dots;
        self.willpower.current = dots.get();
        Ok(self)
    }
}
