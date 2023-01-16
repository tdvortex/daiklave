use crate::sorcery::{
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SpellId,
};

use super::TerrestrialSpell;

pub type AddTerrestrialSorcery = Box<(
    SorceryArchetypeId,
    SorceryArchetype,
    ShapingRitualId,
    ShapingRitual,
    SpellId,
    TerrestrialSpell,
)>;
