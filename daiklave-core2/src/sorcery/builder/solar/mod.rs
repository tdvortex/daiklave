mod with_archetype;
mod with_shaping_ritual;
pub use with_archetype::SolarSorceryBuilderWithArchetype;
pub use with_shaping_ritual::SolarSorceryBuilderWithShapingRitual;

use crate::sorcery::archetype::{AddSorceryArchetype, SorceryArchetypeName};

/// A builder to construct a Solar's ascension to the third and highest circle
/// of sorcery.
pub struct SolarSorceryBuilder;

impl SolarSorceryBuilder {
    /// The Solar continues the archetype they pursued in either the Terrestrial or Celestial circles.
    pub fn existing_archetype(
        self,
        archetype_name: SorceryArchetypeName,
    ) -> SolarSorceryBuilderWithArchetype {
        SolarSorceryBuilderWithArchetype {
            archetype_name,
            archetype: None,
        }
    }

    /// The Solar begins a new archetype.
    pub fn new_archetype(self, archetype: AddSorceryArchetype) -> SolarSorceryBuilderWithArchetype {
        SolarSorceryBuilderWithArchetype {
            archetype_name: archetype.0,
            archetype: Some(archetype.1),
        }
    }
}
