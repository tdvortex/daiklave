use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{sorcery::{
    archetype::{SorceryArchetypeName},
    spell::SpellName,
    ShapingRitualDetails, SorceryArchetypeDetails,
}, merits::merit::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName}};

use super::{TerrestrialSpell};

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
