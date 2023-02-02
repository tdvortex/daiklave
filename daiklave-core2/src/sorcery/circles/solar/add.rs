use crate::sorcery::{
    archetype::SorceryArchetypeName, spell::SpellName, ShapingRitualDetails,
    SorceryArchetypeDetails,
};

use super::SolarSpell;

/// A mutation to upgrade the character from Celestial-level
/// Sorcery to Solar-level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSolarSorcery {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetypeDetails>,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
    pub(crate) control_spell_name: SpellName,
    pub(crate) control_spell: SolarSpell,
}
