use serde::{Serialize, Deserialize};

use crate::{Character, CharacterMutationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Willpower {
    current: u8,
    rating: u8,
}

impl Default for Willpower {
    fn default() -> Self {
        Self { current: 3, rating: 3 }
    }
}

impl Willpower {
    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn rating(&self) -> u8 {
        self.rating
    }
}

impl Character {
    pub fn willpower(&self) -> &Willpower {
        &self.willpower
    }

    pub fn check_set_current_willpower(&mut self, _amount: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_current_willpower(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.current = amount;
        Ok(self)
    }

    pub fn check_set_willpower_rating(&mut self, _dots: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_willpower_rating(&mut self, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.rating = dots;
        self.willpower.current = dots;
        Ok(self)
    }
}