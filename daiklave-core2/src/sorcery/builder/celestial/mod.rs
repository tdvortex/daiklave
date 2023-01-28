mod with_archetype;
mod with_shaping_ritual;
pub use with_archetype::CelestialSorceryBuilderWithArchetype;
pub use with_shaping_ritual::CelestialSorceryBuilderWithShapingRitual;

use crate::sorcery::archetype::{AddSorceryArchetype, SorceryArchetypeName};

/// A builder to construct an Exalt's rise to the second circle of sorcery.
pub struct CelestialSorceryBuilder;

impl CelestialSorceryBuilder {
    /// The Exalt continues on the same archetype as their Terrestrial circle.
    pub fn existing_archetype(
        self,
        archetype_name: SorceryArchetypeName,
    ) -> CelestialSorceryBuilderWithArchetype {
        CelestialSorceryBuilderWithArchetype {
            archetype_name,
            archetype: None,
        }
    }

    /// The Exalt studies sorcery through a second sorcerous archetype.
    pub fn new_archetype(
        self,
        archetype: AddSorceryArchetype,
    ) -> CelestialSorceryBuilderWithArchetype {
        CelestialSorceryBuilderWithArchetype {
            archetype_name: archetype.0,
            archetype: Some(archetype.1),
        }
    }
}
