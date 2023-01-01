use std::collections::HashMap;

use super::{archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, terrestrial_spell::TerrestrialSpell, celestial_spell::CelestialSpell, solar_spell::SolarSpell, sorcery_circle::SorceryCircle, spell::Spell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SolarCircleSorcererView<'source> {
    pub(crate) archetypes: HashMap<SorceryArchetypeId, &'source SorceryArchetype>,
    pub(crate) circle_archetypes: [SorceryArchetypeId; 3],
    pub(crate) shaping_ritual_ids: [ShapingRitualId; 3],
    pub(crate) shaping_rituals: [&'source ShapingRitual; 3],
    pub(crate) terrestrial_control_spell_id: SpellId,
    pub(crate) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    pub(crate) celestial_control_spell_id: SpellId,
    pub(crate) celestial_control_spell: &'source CelestialSpell,
    pub(crate) celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
    pub(crate) solar_control_spell_id: SpellId,
    pub(crate) solar_control_spell: &'source SolarSpell,
    pub(crate) solar_spells: HashMap<SpellId, &'source SolarSpell>,
}



impl<'source> SolarCircleSorcererView<'source> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        if self.circle_archetypes.contains(&id) {
            self.archetypes.get(&id).copied()
        } else {
            None
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> (ShapingRitualId, &'source ShapingRitual) {
        match circle {
            SorceryCircle::Terrestrial => (self.shaping_ritual_ids[0], self.shaping_rituals[0]),
            SorceryCircle::Celestial => (self.shaping_ritual_ids[1], self.shaping_rituals[1]),
            SorceryCircle::Solar => (self.shaping_ritual_ids[2], self.shaping_rituals[2]),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> (SpellId, &'source Spell) {
        match circle {
            SorceryCircle::Terrestrial => (
                self.terrestrial_control_spell_id,
                self.terrestrial_control_spell,
            ),
            SorceryCircle::Celestial => (
                self.celestial_control_spell_id,
                self.celestial_control_spell,
            ),
            SorceryCircle::Solar => (self.solar_control_spell_id, self.solar_control_spell),
        }
    }
}