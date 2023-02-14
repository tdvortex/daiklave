use serde::{Serialize, Deserialize};

use crate::{
    sorcery::{
        archetype::SorceryArchetypeName,
        builder::{TerrestrialSorceryArchetypeBuilder, TerrestrialSorceryBuilderWithArchetype},
        spell::SpellName,
        AddSorcery, AddSorceryArchetype, ShapingRitualDetails, SorceryArchetypeDetails,
    },
    CharacterMutation,
};

use super::TerrestrialSpell;

/// A struct containing all of the details to start the character as a
/// Terrestrial-circle Sorcerer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddTerrestrialSorcery {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) shaping_ritual_summary: String,
    pub(crate) shaping_ritual: ShapingRitualDetails,
    pub(crate) control_spell_name: SpellName,
    pub(crate) control_spell: TerrestrialSpell,
}

impl AddTerrestrialSorcery {
    /// Starts building a new archetype for the circle initiation with the given name.
    pub fn archetype_name(
        name: impl Into<SorceryArchetypeName>,
    ) -> TerrestrialSorceryArchetypeBuilder {
        AddSorcery::terrestrial_circle().archetype_name(name)
    }

    /// Sets the archetype for the circle to an existing archetype.
    pub fn archetype(add_archetype: AddSorceryArchetype) -> TerrestrialSorceryBuilderWithArchetype {
        TerrestrialSorceryBuilderWithArchetype {
            archetype_name: add_archetype.name,
            archetype: add_archetype.archetype,
        }
    }
}

impl From<AddTerrestrialSorcery> for CharacterMutation {
    fn from(add_terrestrial: AddTerrestrialSorcery) -> Self {
        AddSorcery::from(add_terrestrial).into()
    }
}
