use crate::sorcery::{
    spell::SpellId, ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
};

use super::SolarSpell;

/// A struct containing all of the details to upgrade from Celestial-level
/// Sorcery to Solar-level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSolarSorcery {
    /// The Id of the archetype used for the Solar circle. May be the same
    /// as either the Terrestrial or Celestial circle, or entirely new.
    pub archetype_id: SorceryArchetypeId,
    /// If necessary, the sorcery archetype added for the Solar Circle.
    /// Ignored if the Solar archetype id matches either prior circle.
    pub archetype: Option<SorceryArchetype>,
    /// The Id of the shaping ritual for the Solar circle. This must be
    /// unique.
    pub shaping_ritual_id: ShapingRitualId,
    /// The shaping ritual for the Solar circle.
    pub shaping_ritual: ShapingRitual,
    /// The Id for the Solar Control Spell.
    pub control_spell_id: SpellId,
    /// The Solar Control Spell.
    pub control_spell: SolarSpell,
}
