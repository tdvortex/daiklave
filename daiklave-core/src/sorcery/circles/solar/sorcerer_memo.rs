use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    merits::merit::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName},
    sorcery::{
        circles::{celestial::CelestialSpell, terrestrial::TerrestrialSpell},
        spell::SpellName,
        ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetypeName,
    },
};

use super::{sorcerer::SolarCircleSorcerer, SolarSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetypes: HashMap<
        SorceryArchetypeName,
        (
            SorceryArchetypeDetails,
            HashMap<SorceryArchetypeMeritName, SorceryArchetypeMeritDetails>,
        ),
    >,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeName; 3],
    pub(in crate::sorcery::circles) shaping_ritual_names: [String; 3],
    pub(in crate::sorcery::circles) shaping_rituals: [ShapingRitualDetails; 3],
    pub(in crate::sorcery::circles) terrestrial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) terrestrial_control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellName, TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) celestial_control_spell: CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellName, CelestialSpell>,
    pub(in crate::sorcery::circles) solar_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) solar_control_spell: SolarSpell,
    pub(in crate::sorcery::circles) solar_spells: HashMap<SpellName, SolarSpell>,
}

impl From<&SolarCircleSorcerer<'_>> for SolarCircleSorcererMemo {
    fn from(value: &SolarCircleSorcerer<'_>) -> Self {
        Self {
            archetypes: value
                .archetypes
                .iter()
                .map(|(name, (archetype_details, merit_map))| {
                    (
                        (*name).into(),
                        (
                            (*archetype_details).to_owned(),
                            merit_map
                                .iter()
                                .map(|(name, details)| ((*name).into(), (*details).to_owned()))
                                .collect(),
                        ),
                    )
                })
                .collect(),
            circle_archetypes: value.circle_archetypes.map(|name| name.into()),
            shaping_ritual_names: value.shaping_ritual_names.map(|name| name.into()),
            shaping_rituals: [
                value.shaping_rituals[0].to_owned(),
                value.shaping_rituals[1].to_owned(),
                value.shaping_rituals[2].to_owned(),
            ],
            terrestrial_control_spell_name: value.terrestrial_control_spell_name.into(),
            terrestrial_control_spell: value.terrestrial_control_spell.to_owned(),
            terrestrial_spells: value
                .terrestrial_spells
                .iter()
                .map(|(name, spell)| ((*name).into(), (*spell).to_owned()))
                .collect(),
            celestial_control_spell_name: value.celestial_control_spell_name.into(),
            celestial_control_spell: value.celestial_control_spell.to_owned(),
            celestial_spells: value
                .celestial_spells
                .iter()
                .map(|(name, spell)| ((*name).into(), (*spell).to_owned()))
                .collect(),
            solar_control_spell_name: value.solar_control_spell_name.into(),
            solar_control_spell: value.solar_control_spell.to_owned(),
            solar_spells: value
                .solar_spells
                .iter()
                .map(|(name, spell)| ((*name).into(), (*spell).to_owned()))
                .collect(),
        }
    }
}
