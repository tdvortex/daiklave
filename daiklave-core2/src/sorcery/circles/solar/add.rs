use crate::sorcery::{
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SpellId,
};

use super::SolarSpell;

pub type AddSolarSorcery = Box<(
    SorceryArchetypeId,
    Option<SorceryArchetype>,
    ShapingRitualId,
    ShapingRitual,
    SpellId,
    SolarSpell,
)>;
