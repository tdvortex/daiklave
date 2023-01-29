use crate::sorcery::{
    archetype::{SorceryArchetypeName},
    spell::SpellName,
    ShapingRitual, SorceryArchetype,
};

use super::CelestialSpell;

/// A struct containing all of the details to upgrade from Terrestrial-level
/// Sorcery to Celestial-level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddCelestialSorcery {
    /// The name of the archetype used for the Celestial circle. May be the same
    /// as the Terrestrial archetype or different.
    pub archetype_name: SorceryArchetypeName,
    /// If necessary, the sorcery archetype added for the Celestial Circle.
    /// Ignored if the Celestial archetype id is the same as Terrestrial.
    pub archetype: Option<SorceryArchetype>,
    /// A short description of the shaping ritual for the Celestial
    /// circle. This must be unique.
    pub shaping_ritual_summary: String,
    /// The shaping ritual for the Celestial circle.
    pub shaping_ritual: ShapingRitual,
    /// The name of the Celestial Control Spell.
    pub control_spell_name: SpellName,
    /// The spell selected as the Control Spell for the Celestial Circle.
    pub control_spell: CelestialSpell,
}
