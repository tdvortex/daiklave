mod with_archetype;
mod with_shaping_ritual;
pub use with_archetype::TerrestrialSorceryBuilderWithArchetype;
pub use with_shaping_ritual::TerrestrialSorceryBuilderWithShapingRitual;

use crate::sorcery::archetype::AddSorceryArchetype;

/// A builder to create a new sorcerer of the first circle.
pub struct TerrestrialSorceryBuilder;

impl TerrestrialSorceryBuilder {
    /// Sets the archetype the sorcerer intiates into.
    pub fn archetype(
        self,
        archetype: AddSorceryArchetype,
    ) -> TerrestrialSorceryBuilderWithArchetype {
        TerrestrialSorceryBuilderWithArchetype {
            archetype_name: archetype.name,
            archetype: archetype.archetype,
        }
    }
}
