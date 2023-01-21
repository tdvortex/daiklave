use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::{
    circles::{celestial::CelestialSpell, terrestrial::TerrestrialSpell},
    spell::SpellId,
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryArchetypeMerit,
    SorceryArchetypeMeritId,
};

use super::{sorcerer::SolarCircleSorcerer, SolarSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetypes: HashMap<
        SorceryArchetypeId,
        (
            SorceryArchetype,
            HashMap<SorceryArchetypeMeritId, SorceryArchetypeMerit>,
        ),
    >,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeId; 3],
    pub(in crate::sorcery::circles) shaping_ritual_ids: [ShapingRitualId; 3],
    pub(in crate::sorcery::circles) shaping_rituals: [ShapingRitual; 3],
    pub(in crate::sorcery::circles) terrestrial_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) terrestrial_control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) celestial_control_spell: CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellId, CelestialSpell>,
    pub(in crate::sorcery::circles) solar_control_spell_id: SpellId,
    pub(in crate::sorcery::circles) solar_control_spell: SolarSpell,
    pub(in crate::sorcery::circles) solar_spells: HashMap<SpellId, SolarSpell>,
}

impl<'source> SolarCircleSorcererMemo {
    pub fn as_ref(&'source self) -> SolarCircleSorcerer<'source> {
        SolarCircleSorcerer {
            archetypes: self
                .archetypes
                .iter()
                .map(|(k, (archetype, merits))| {
                    (
                        *k,
                        (archetype, merits.iter().map(|(k, v)| (*k, v)).collect()),
                    )
                })
                .collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: {
                self.shaping_rituals
                    .iter()
                    .enumerate()
                    .fold([None; 3], |mut opt_arr, (i, el)| {
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
            solar_control_spell_id: self.solar_control_spell_id,
            solar_control_spell: &self.solar_control_spell,
            solar_spells: self.solar_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}
