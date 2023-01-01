use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId};

use super::{TerrestrialSpell, sorcerer_view::TerrestrialCircleSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetype_id: SorceryArchetypeId,
    pub(in crate::sorcery::circles) archetype: SorceryArchetype,
    pub(in crate::sorcery::circles) shaping_ritual_id: ShapingRitualId,
    pub(in crate::sorcery::circles) shaping_ritual: ShapingRitual,
    pub(in crate::sorcery::circles) control_spell_id: SpellId,
    pub(in crate::sorcery::circles) control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) other_spells: HashMap<SpellId, TerrestrialSpell>,
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