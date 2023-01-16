use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::{
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryArchetypeMerit,
    SorceryArchetypeMeritId, SpellId,
};

use super::{sorcerer::TerrestrialCircleSorcerer, TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    pub archetype_id: SorceryArchetypeId,
    pub archetype: SorceryArchetype,
    pub archetype_merits: HashMap<SorceryArchetypeMeritId, SorceryArchetypeMerit>,
    pub shaping_ritual_id: ShapingRitualId,
    pub shaping_ritual: ShapingRitual,
    pub control_spell_id: SpellId,
    pub control_spell: TerrestrialSpell,
    pub other_spells: HashMap<SpellId, TerrestrialSpell>,
}

impl<'source> TerrestrialCircleSorcererMemo {
    pub fn as_ref(&'source self) -> TerrestrialCircleSorcerer<'source> {
        TerrestrialCircleSorcerer {
            archetype_id: self.archetype_id,
            archetype: &self.archetype,
            archetype_merits: self.archetype_merits.iter().map(|(k, v)| (*k, v)).collect(),
            shaping_ritual_id: self.shaping_ritual_id,
            shaping_ritual: &self.shaping_ritual,
            control_spell_id: self.control_spell_id,
            control_spell: &self.control_spell,
            other_spells: self.other_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}
