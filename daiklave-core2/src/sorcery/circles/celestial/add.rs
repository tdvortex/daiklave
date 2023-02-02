use crate::sorcery::{
    archetype::SorceryArchetypeName, spell::SpellName, ShapingRitualDetails,
    SorceryArchetypeDetails,
};

use super::CelestialSpell;

/// A mutation to upgrade the character from Terrestrial-level
/// Sorcery to Celestial-level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddCelestialSorcery {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetypeDetails>,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
    pub(crate) control_spell_name: SpellName,
    pub(crate) control_spell: CelestialSpell,
}
