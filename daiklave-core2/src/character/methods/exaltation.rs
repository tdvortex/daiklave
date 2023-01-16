use crate::{Character, CharacterMutationError};

impl<'source> Character<'source> {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exaltation.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exaltation.is_exalted()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exaltation.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exaltation.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }
}
