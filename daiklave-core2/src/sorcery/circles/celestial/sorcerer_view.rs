use std::collections::HashMap;

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell, sorcery_circle::SorceryCircle}, Spell};

use super::{spell::CelestialSpell, sorcerer_memo::CelestialCircleSorcererMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CelestialCircleSorcererView<'source> {
    pub(in crate::sorcery::circles) archetypes: HashMap<SorceryArchetypeId, &'source SorceryArchetype>,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeId; 2],
    pub(in crate::sorcery::circles) shaping_ritual_ids: [ShapingRitualId; 2],
    pub(in crate::sorcery::circles) shaping_rituals: [&'source ShapingRitual; 2],
    pub(in crate::sorcery::circles) terrestrial_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) celestial_control_spell: &'source CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
}

impl<'source> CelestialCircleSorcererView<'source> {
    pub fn as_memo(&self) -> CelestialCircleSorcererMemo {
        CelestialCircleSorcererMemo {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, (*v).to_owned())).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: {
                self.shaping_rituals.map(|ptr| ptr.to_owned())
            },
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: self.terrestrial_control_spell.to_owned(),
            terrestrial_spells: self.terrestrial_spells.iter().map(|(k, v)| (*k, (*v).to_owned())).collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: self.celestial_control_spell.to_owned(),
            celestial_spells: self.celestial_spells.iter().map(|(k, v)| (*k, (*v).to_owned())).collect(),
        }
    }


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