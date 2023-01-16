use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId};

use super::SolarSpell;

pub type AddSolarSorcery = Box<(
    SorceryArchetypeId,
    Option<SorceryArchetype>,
    ShapingRitualId,
    ShapingRitual,
    SpellId,
    SolarSpell
)>;