use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId};

use super::{TerrestrialSpell, sorcerer_view::TerrestrialCircleSorcererView};

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

impl<'source> TerrestrialCircleSorcererMemo {
    pub fn as_ref(&'source self) -> TerrestrialCircleSorcererView<'source> {
        TerrestrialCircleSorcererView {
            archetype_id: self.archetype_id,
            archetype: &self.archetype,
            shaping_ritual_id: self.shaping_ritual_id,
            shaping_ritual: &self.shaping_ritual,
            control_spell_id: self.control_spell_id,
            control_spell: &self.control_spell,
            other_spells: self.other_spells.iter().map(|(k, v)| (*k, v)).collect()
        }
    }
}