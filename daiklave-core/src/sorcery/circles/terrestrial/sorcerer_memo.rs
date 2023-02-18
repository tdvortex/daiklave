use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    merits::merit::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName},
    sorcery::{
        archetype::SorceryArchetypeName, spell::SpellName, ShapingRitualDetails,
        SorceryArchetypeDetails,
    },
};

use super::{sorcerer::TerrestrialCircleSorcerer, TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    pub archetype_name: SorceryArchetypeName,
    pub archetype: SorceryArchetypeDetails,
    pub archetype_merits: HashMap<SorceryArchetypeMeritName, SorceryArchetypeMeritDetails>,
    pub shaping_ritual_name: String,
    pub shaping_ritual: ShapingRitualDetails,
    pub control_spell_name: SpellName,
    pub control_spell: TerrestrialSpell,
    pub other_spells: HashMap<SpellName, TerrestrialSpell>,
}

impl From<&TerrestrialCircleSorcerer<'_>> for TerrestrialCircleSorcererMemo {
    fn from(terrestrial: &TerrestrialCircleSorcerer<'_>) -> Self {
        Self {
            archetype_name: terrestrial.archetype_name.into(),
            archetype: terrestrial.archetype.to_owned(),
            archetype_merits: terrestrial
                .archetype_merits
                .iter()
                .map(|(name, &details)| ((*name).into(), details.to_owned()))
                .collect(),
            shaping_ritual_name: terrestrial.shaping_ritual_name.into(),
            shaping_ritual: terrestrial.shaping_ritual.to_owned(),
            control_spell_name: terrestrial.control_spell_name.into(),
            control_spell: terrestrial.control_spell.to_owned(),
            other_spells: terrestrial
                .other_spells
                .iter()
                .map(|(name, &spell)| ((*name).into(), spell.to_owned()))
                .collect(),
        }
    }
}
