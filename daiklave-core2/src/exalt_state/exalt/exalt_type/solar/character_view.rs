use crate::{CharacterMutationError, CharacterView};

use super::{Solar, SolarView};

impl<'source> CharacterView<'source> {
    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&self) -> Option<&SolarView> {
        self.exalt_state.solar_traits()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(&self, solar_traits: &Solar) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(
        &mut self,
        solar_traits: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_solar(solar_traits)?;
        if self.is_mortal() {
            let new_willpower_rating = self.willpower().rating() + 2;
            self.set_willpower_rating(new_willpower_rating)?;
        }
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }

    pub(crate) fn check_set_solar_view(
        &self,
        solar_view: &SolarView,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar_view(solar_view)
    }

    pub(crate) fn set_solar_view(
        &mut self,
        solar_view: SolarView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_solar_view(&solar_view)?;
        if self.is_mortal() {
            let new_willpower_rating = self.willpower().rating() + 2;
            self.set_willpower_rating(new_willpower_rating)?;
        }
        self.exalt_state.set_solar_view(solar_view)?;
        Ok(self)
    }
}
