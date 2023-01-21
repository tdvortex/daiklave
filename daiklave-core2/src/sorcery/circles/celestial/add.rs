use crate::sorcery::{
    spell::SpellId, ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
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
