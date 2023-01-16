use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, SpellId, ShapingRitual};

use super::CelestialSpell;

pub type AddCelestialSorcery = Box<(
    SorceryArchetypeId,
    Option<SorceryArchetype>,
    ShapingRitualId,
    ShapingRitual,
    SpellId,
    CelestialSpell
)>;