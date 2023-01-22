use crate::{
    abilities::AbilityNameVanilla,
    attributes::AttributeName,
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery,
            solar::AddSolarSorcery,
            terrestrial::{AddTerrestrialSorcery, AddTerrestrialSorceryView},
        },
        Sorcery, SorceryArchetypeId, SorceryArchetypeMerit, SorceryArchetypeMeritId, spell::{SpellMutation, SpellId},
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// The character's Sorcery abilities, if any.
    pub fn sorcery(&'view self) -> Option<Sorcery<'view, 'source>> {
        self.exaltation.sorcery()
    }

    /// If the character was not already a sorcerer, adds the first circle of
    /// sorcery.
    pub fn add_terrestrial_sorcery(
        &mut self,
        add_terrestrial: &'source AddTerrestrialSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.add_terrestrial_sorcery_view(add_terrestrial.as_ref())
    }

    pub(crate) fn add_terrestrial_sorcery_view(
        &mut self,
        add_terrestrial: AddTerrestrialSorceryView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_terrestrial_sorcery(
            add_terrestrial,
            self.abilities().get(AbilityNameVanilla::Occult).dots(),
            self.attributes().get(AttributeName::Intelligence).dots(),
        )?;
        Ok(self)
    }

    /// Removes Terrestrial circle sorcery from the character.
    pub fn remove_terrestrial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_terrestrial_sorcery()?;
        Ok(self)
    }

    /// Upgrades the character from Terrestrial to Celestial sorcery.
    pub fn add_celestial_sorcery(
        &mut self,
        add_celestial: &'source AddCelestialSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_celestial_sorcery(
            add_celestial,
            self.abilities().get(AbilityNameVanilla::Occult).dots(),
            self.attributes().get(AttributeName::Intelligence).dots(),
            self.essence().map_or(1, |essence| essence.rating()),
        )?;
        Ok(self)
    }

    /// Removes Celestial circle sorcery from the character.
    pub fn remove_celestial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_celestial_sorcery()?;
        Ok(self)
    }

    /// Upgrades the character from Celestial to Solar sorcery.
    pub fn add_solar_sorcery(
        &mut self,
        add_solar: &'source AddSolarSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_solar_sorcery(
            add_solar,
            self.abilities().get(AbilityNameVanilla::Occult).dots(),
            self.essence().map_or(1, |essence| essence.rating()),
        )?;
        Ok(self)
    }

    /// Removes Solar circle sorcery from the character.
    pub fn remove_solar_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_solar_sorcery()?;
        Ok(self)
    }

    /// Adds a merit to a Sorcery Archetype owned by the character
    pub fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_id: SorceryArchetypeId,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_sorcery_archetype_merit(
            sorcery_archetype_id,
            sorcery_archetype_merit_id,
            sorcery_archetype_merit,
        )?;
        Ok(self)
    }

    /// Removes a merit from a Sorcery Archetype owned by a character
    pub fn remove_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation
            .remove_sorcery_archetype_merit(sorcery_archetype_merit_id)?;
        Ok(self)
    }

    /// Adds a Spell to the character.
    pub fn add_spell(&mut self, spell_id: SpellId, spell: &'source SpellMutation) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Removes a Spell from the character. Control Spells cannot be removed.
    pub fn remove_spell(&mut self, spell_id: SpellId) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}
