use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId};

use super::TerrestrialSpell;

pub type AddTerrestrialSorcery = Box<(SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, TerrestrialSpell)>;