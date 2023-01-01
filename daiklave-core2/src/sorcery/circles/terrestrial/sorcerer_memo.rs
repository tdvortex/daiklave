use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId};

use super::{TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    archetype_id: SorceryArchetypeId,
    archetype: SorceryArchetype,
    shaping_ritual_id: ShapingRitualId,
    shaping_ritual: ShapingRitual,
    control_spell_id: SpellId,
    control_spell: TerrestrialSpell,
    other_spells: HashMap<SpellId, TerrestrialSpell>,
}