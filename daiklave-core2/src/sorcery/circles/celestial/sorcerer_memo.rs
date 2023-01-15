use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::{
    circles::terrestrial::TerrestrialSpell, ShapingRitual, ShapingRitualId, SorceryArchetype,
    SorceryArchetypeId, SpellId, archetype::SorceryArchetypeMeritId, SorceryArchetypeMerit,
};

use super::{sorcerer::CelestialCircleSorcerer, spell::CelestialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CelestialCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetypes: HashMap<SorceryArchetypeId, (SorceryArchetype, HashMap<SorceryArchetypeMeritId, SorceryArchetypeMerit>)>,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeId; 2],
    pub(in crate::sorcery::circles) shaping_ritual_ids: [ShapingRitualId; 2],
    pub(in crate::sorcery::circles) shaping_rituals: [ShapingRitual; 2],
    pub(in crate::sorcery::circles) terrestrial_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) terrestrial_control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) celestial_control_spell: CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellId, CelestialSpell>,
}

impl<'source> CelestialCircleSorcererMemo {
    pub fn as_ref(&'source self) -> CelestialCircleSorcerer<'source> {
        CelestialCircleSorcerer {
            archetypes: self.archetypes.iter().map(|(k, (archetype, merits))| {
                (*k, (archetype, merits.iter().map(|(k, v)| {
                    (*k, v)
                }).collect()))
            }).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: {
                self.shaping_rituals
                    .iter()
                    .enumerate()
                    .fold([None; 2], |mut opt_arr, (i, el)| {
                        opt_arr[i] = Some(el);
                        opt_arr
                    })
                    .map(|opt| opt.unwrap())
            },
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: &self.terrestrial_control_spell,
            terrestrial_spells: self
                .terrestrial_spells
                .iter()
                .map(|(k, v)| (*k, v))
                .collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: &self.celestial_control_spell,
            celestial_spells: self.celestial_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}
