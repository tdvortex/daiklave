use crate::{Character, CharacterMutationError, exaltation::{Exaltation, exalt::{exalt_type::ExaltType}}};

impl<'source> Character<'source> {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        matches!(self.exaltation, Exaltation::Mortal(_))
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        matches!(self.exaltation, Exaltation::Exalt(_))
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

    /// Returns the character's Exalt Type, if they are Exalted.
    pub fn exalt_type(&self) -> Option<&ExaltType<'source>> {
        match &self.exaltation {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => {
                Some(exalt.exalt_type())
            }
        }
    }
}
