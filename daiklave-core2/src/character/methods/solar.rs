use crate::{Character, exaltation::exalt::exalt_type::solar::{Solar, NewSolar}, CharacterMutationError};

impl<'source> Character<'source> {
    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exaltation.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&'source self) -> Option<&Solar> {
        self.exaltation.solar_traits()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(&self, solar: &'source NewSolar) -> Result<(), CharacterMutationError> {
        self.exaltation.check_set_solar(solar.0.as_ref())
    }

    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(
        &mut self,
        solar: &'source NewSolar,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            let new_willpower_rating = self.willpower().rating() + 2;
            self.set_willpower_rating(new_willpower_rating)?;
        }
        self.exaltation.set_solar(solar.0.as_ref())?;
        Ok(self)
    }
}