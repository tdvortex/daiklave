use crate::sorcery::{
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SpellId,
};

use super::CelestialSpell;

pub type AddCelestialSorcery = Box<(
    SorceryArchetypeId,
    Option<SorceryArchetype>,
    ShapingRitualId,
    ShapingRitual,
    SpellId,
    CelestialSpell,
)>;
