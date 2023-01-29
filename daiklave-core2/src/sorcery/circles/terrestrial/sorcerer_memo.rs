use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::{
    archetype::{SorceryArchetypeName},
    spell::SpellName,
    ShapingRitual, SorceryArchetype, SorceryArchetypeMerit,
};

use super::{TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    pub archetype_name: SorceryArchetypeName,
    pub archetype: SorceryArchetype,
    pub archetype_merits: HashMap<String, SorceryArchetypeMerit>,
    pub shaping_ritual_name: String,
    pub shaping_ritual: ShapingRitual,
    pub control_spell_name: SpellName,
    pub control_spell: TerrestrialSpell,
    pub other_spells: HashMap<SpellName, TerrestrialSpell>,
}
