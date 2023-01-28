use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::{
    archetype::{ShapingRitualSummary, SorceryArchetypeName},
    spell::SpellName,
    ShapingRitual, SorceryArchetype, SorceryArchetypeMerit, SorceryArchetypeMeritId,
};

use super::{sorcerer::TerrestrialCircleSorcerer, TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    pub archetype_name: SorceryArchetypeName,
    pub archetype: SorceryArchetype,
    pub archetype_merits: HashMap<SorceryArchetypeMeritId, SorceryArchetypeMerit>,
    pub shaping_ritual_name: ShapingRitualSummary,
    pub shaping_ritual: ShapingRitual,
    pub control_spell_name: SpellName,
    pub control_spell: TerrestrialSpell,
    pub other_spells: HashMap<SpellName, TerrestrialSpell>,
}

impl<'source> TerrestrialCircleSorcererMemo {
    pub fn as_ref(&'source self) -> TerrestrialCircleSorcerer<'source> {
        TerrestrialCircleSorcerer {
            archetype_name: self.archetype_name.as_str(),
            archetype: &self.archetype,
            archetype_merits: self.archetype_merits.iter().map(|(k, v)| (*k, v)).collect(),
            shaping_ritual_name: self.shaping_ritual_name.as_str(),
            shaping_ritual: &self.shaping_ritual,
            control_spell_name: self.control_spell_name.as_str(),
            control_spell: &self.control_spell,
            other_spells: self
                .other_spells
                .iter()
                .map(|(k, v)| (k.as_str(), v))
                .collect(),
        }
    }
}
