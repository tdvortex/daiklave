use crate::sorcery::{
    archetype::{ShapingRitualSummary, SorceryArchetypeName},
    spell::SpellName,
    ShapingRitual, SorceryArchetype,
};

use super::TerrestrialSpell;

/// A struct containing all of the details to start the character as a
/// Terrestrial-circle Sorcerer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddTerrestrialSorcery {
    /// The name of the Sorcery Archetype they initiate into.
    pub archetype_name: SorceryArchetypeName,
    /// The Archetype they inititate into.
    pub archetype: SorceryArchetype,
    /// The name of their first Shaping Ritual (or a short blurb to describe
    /// it.)
    pub shaping_ritual_name: ShapingRitualSummary,
    /// The first Shaping Ritual.
    pub shaping_ritual: ShapingRitual,
    /// The name of their Terrestrial Control Spell.
    pub control_spell_name: SpellName,
    /// Their Terrestrial Control Spell.
    pub control_spell: TerrestrialSpell,
}

impl<'source> AddTerrestrialSorcery {
    pub(crate) fn as_ref(&'source self) -> AddTerrestrialSorceryView<'source> {
        AddTerrestrialSorceryView {
            archetype_name: self.archetype_name.as_str(),
            archetype: &self.archetype,
            shaping_ritual_name: self.shaping_ritual_name.as_str(),
            shaping_ritual: &self.shaping_ritual,
            control_spell_name: self.control_spell_name.as_str(),
            control_spell: &self.control_spell,
        }
    }
}

pub(crate) struct AddTerrestrialSorceryView<'source> {
    pub archetype_name: &'source str,
    pub archetype: &'source SorceryArchetype,
    pub shaping_ritual_name: &'source str,
    pub shaping_ritual: &'source ShapingRitual,
    pub control_spell_name: &'source str,
    pub control_spell: &'source TerrestrialSpell,
}
