use crate::{sorcery::{
    archetype::{SorceryArchetypeName},
    spell::SpellName,
    ShapingRitualDetails, SorceryArchetypeDetails, AddSorcery,
}, CharacterMutation};

use super::TerrestrialSpell;

/// A struct containing all of the details to start the character as a
/// Terrestrial-circle Sorcerer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddTerrestrialSorcery {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
    pub(crate) control_spell_name: SpellName,
    pub(crate) control_spell: TerrestrialSpell,
}

impl From<AddTerrestrialSorcery> for CharacterMutation {
    fn from(add_terrestrial: AddTerrestrialSorcery) -> Self {
        AddSorcery::from(add_terrestrial).into()
    }
}