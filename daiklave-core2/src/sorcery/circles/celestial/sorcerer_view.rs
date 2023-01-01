use std::collections::HashMap;

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell, sorcery_circle::SorceryCircle}, Spell};

use super::spell::CelestialSpell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CelestialCircleSorcererView<'source> {
    pub(crate) archetypes: HashMap<SorceryArchetypeId, &'source SorceryArchetype>,
    pub(crate) circle_archetypes: [SorceryArchetypeId; 2],
    pub(crate) shaping_ritual_ids: [ShapingRitualId; 2],
    pub(crate) shaping_rituals: [&'source ShapingRitual; 2],
    pub(crate) terrestrial_control_spell_id: SpellId,
    pub(crate) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    pub(crate) celestial_control_spell_id: SpellId,
    pub(crate) celestial_control_spell: &'source CelestialSpell,
    pub(crate) celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
}

impl<'source> CelestialCircleSorcererView<'source> {
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
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match circle {
            SorceryCircle::Terrestrial => {
                Some((self.shaping_ritual_ids[0], self.shaping_rituals[0]))
            }
            SorceryCircle::Celestial => Some((self.shaping_ritual_ids[1], self.shaping_rituals[1])),
            SorceryCircle::Solar => None,
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        match circle {
            SorceryCircle::Terrestrial => Some((
                self.terrestrial_control_spell_id,
                self.terrestrial_control_spell,
            )),
            SorceryCircle::Celestial => Some((
                self.celestial_control_spell_id,
                self.celestial_control_spell,
            )),
            SorceryCircle::Solar => None,
        }
    }
}