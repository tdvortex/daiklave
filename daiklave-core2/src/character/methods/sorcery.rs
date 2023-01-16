use crate::{
    sorcery::{
        ShapingRitual, ShapingRitualId, Sorcery, SorceryArchetype, SorceryArchetypeId, SpellId,
        TerrestrialSpell,
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
        )?;
        Ok(self)
    }

    /// Checks if the character can have Terrestrial Circle sorcery added.
    pub fn check_add_terrestrial_sorcery(
        &self,
        archetype_id: SorceryArchetypeId,
        archetype: &'source SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source TerrestrialSpell,
    ) -> Result<(), CharacterMutationError> {
        self.exaltation.check_add_terrestrial_sorcery(
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
        )?;
        Ok(())
    }

    /// Removes Terrestrial circle sorcery from the character.
    pub fn remove_terrestrial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_terrestrial_sorcery()?;
        Ok(self)
    }

    /// Checks if Terrestrial circle sorcery can be removed from the character.
    pub fn check_remove_terrestrial_sorcery(&self) -> Result<(), CharacterMutationError> {
        self.exaltation.check_remove_terrestrial_sorcery()
    }
}
