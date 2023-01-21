use crate::sorcery::{
    spell::SpellId, ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
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
