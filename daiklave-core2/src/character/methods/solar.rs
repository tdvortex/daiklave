use crate::{
    exaltation::exalt::exalt_type::solar::{NewSolar, Solar, charm::{SolarCharm, SolarCharmId}},
    Character, CharacterMutationError, charms::charm::{EclipseCharm, SpiritCharmId},
};

impl<'source> Character<'source> {
    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exaltation.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&'source self) -> Option<&Solar> {
        self.exaltation.solar_traits()
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

    /// Adds a Solar Charm to the character.
    pub fn add_solar_charm(&mut self, solar_charm_id: SolarCharmId, charm: &'source SolarCharm) -> Result<&mut Self, CharacterMutationError> {        
        todo!()
    }

    /// Removes a Solar Charm from the character.
    pub fn remove_solar_charm(&mut self, solar_charm_id: SolarCharmId) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Adds an Eclipse Charm to the character.
    pub fn add_eclipse_charm(&mut self, charm_id: SpiritCharmId, charm: &'source EclipseCharm) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Removes a Spirit Charm from the character.
    pub fn remove_spirit_charm(&mut self, charm_id: SpiritCharmId) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}
