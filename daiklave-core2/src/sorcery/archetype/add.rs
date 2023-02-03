use crate::sorcery::builder::TerrestrialSorceryBuilderWithArchetype;

use super::{SorceryArchetypeDetails, SorceryArchetypeName};

/// A mutation to add a sorcery archetype, as part of a sorcerous initiation.
pub struct AddSorceryArchetype {
    pub(crate) name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
}

impl AddSorceryArchetype {
    /// Starts building a mutation to add the Terrestial Circle of sorcery to
    /// a character, using this archetype as their initiation.
    pub fn into_terrestrial_circle(self) -> TerrestrialSorceryBuilderWithArchetype {
        TerrestrialSorceryBuilderWithArchetype {
            archetype_name: self.name,
            archetype: self.archetype,
        }
    }
}
