use crate::sorcery::{
    spell::SpellId, ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
};

use super::TerrestrialSpell;

/// A struct containing all of the details to start the character as a
/// Terrestrial-circle Sorcerer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddTerrestrialSorcery {
    /// The Id of the Sorcery Archetype they initiate into.
    pub archetype_id: SorceryArchetypeId,
    /// The Archetype they inititate into.
    pub archetype: SorceryArchetype,
    /// The Id of their first Shaping Ritual.
    pub shaping_ritual_id: ShapingRitualId,
    /// The first Shaping Ritual.
    pub shaping_ritual: ShapingRitual,
    /// The Id of their Terrestrial Control Spell.
    pub control_spell_id: SpellId,
    /// Their Terrestrial Control Spell.
    pub control_spell: TerrestrialSpell,
}

impl<'source> AddTerrestrialSorcery {
    pub(crate) fn as_ref(&'source self) -> AddTerrestrialSorceryView<'source> {
        AddTerrestrialSorceryView {
            archetype_id: self.archetype_id,
            archetype: &self.archetype,
            shaping_ritual_id: self.shaping_ritual_id,
            shaping_ritual: &self.shaping_ritual,
            control_spell_id: self.control_spell_id,
            control_spell: &self.control_spell,
        }
    }
}

pub(crate) struct AddTerrestrialSorceryView<'source> {
    pub archetype_id: SorceryArchetypeId,
    pub archetype: &'source SorceryArchetype,
    pub shaping_ritual_id: ShapingRitualId,
    pub shaping_ritual: &'source ShapingRitual,
    pub control_spell_id: SpellId,
    pub control_spell: &'source TerrestrialSpell,
}
