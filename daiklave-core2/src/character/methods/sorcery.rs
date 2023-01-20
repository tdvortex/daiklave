use crate::{
    sorcery::{
        CelestialSpell, ShapingRitual, ShapingRitualId, SolarSpell, Sorcery, SorceryArchetype,
        SorceryArchetypeId, SorceryArchetypeMerit, SorceryArchetypeMeritId, SpellId,
        TerrestrialSpell,
    },
    Character, CharacterMutationError, abilities::AbilityNameVanilla, attributes::AttributeName,
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
        archetype_id: SorceryArchetypeId,
        archetype: &'source SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source TerrestrialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_terrestrial_sorcery(
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
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
        archetype_id: SorceryArchetypeId,
        archetype: Option<&'source SorceryArchetype>,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source CelestialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_celestial_sorcery(
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
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
        archetype_id: SorceryArchetypeId,
        archetype: Option<&'source SorceryArchetype>,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source SolarSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_solar_sorcery(
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
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
}
