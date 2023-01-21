use crate::sorcery::{
    spell::SpellId, ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
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
