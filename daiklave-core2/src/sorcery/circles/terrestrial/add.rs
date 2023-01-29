use crate::sorcery::{
    archetype::{SorceryArchetypeName},
    spell::SpellName,
    ShapingRitualDetails, SorceryArchetypeDetails,
};

use super::TerrestrialSpell;

/// A struct containing all of the details to start the character as a
/// Terrestrial-circle Sorcerer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddTerrestrialSorcery {
    /// The name of the Sorcery Archetype they initiate into.
    pub archetype_name: SorceryArchetypeName,
    /// The Archetype they inititate into.
    pub archetype: SorceryArchetypeDetails,
    /// A short summary of their first Shaping Ritual.
    pub shaping_ritual_summary: String,
    /// The first Shaping Ritual.
    pub shaping_ritual: ShapingRitualDetails,
    /// The name of their Terrestrial Control Spell.
    pub control_spell_name: SpellName,
    /// Their Terrestrial Control Spell.
    pub control_spell: TerrestrialSpell,
}

pub(crate) struct AddTerrestrialSorceryView<'source> {
    pub archetype_name: &'source str,
    pub archetype: &'source SorceryArchetypeDetails,
    pub shaping_ritual_name: &'source str,
    pub shaping_ritual: &'source ShapingRitualDetails,
    pub control_spell_name: &'source str,
    pub control_spell: &'source TerrestrialSpell,
}
