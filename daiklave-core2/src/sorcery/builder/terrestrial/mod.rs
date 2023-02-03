mod archetype_builder;
mod control_spell_builder;
mod shaping_ritual_builder;
mod with_archetype;
mod with_shaping_ritual;
pub use archetype_builder::TerrestrialSorceryArchetypeBuilder;
pub use control_spell_builder::{
    TerrestrialControlSpellBuilder, TerrestrialControlSpellBuilderWithDescription,
    TerrestrialControlSpellBuilderWithDuration, TerrestrialControlSpellBuilderWithMoteCost,
    TerrestrialControlSpellBuilderWithWillpower,
};
pub use shaping_ritual_builder::TerrestrialShapingRitualBuilder;
pub use with_archetype::TerrestrialSorceryBuilderWithArchetype;
pub use with_shaping_ritual::TerrestrialSorceryBuilderWithShapingRitual;

use crate::sorcery::{archetype::AddSorceryArchetype, SorceryArchetypeName};

/// A builder to create a new sorcerer of the first circle.
pub struct TerrestrialSorceryBuilder;

impl TerrestrialSorceryBuilder {
    /// Starts building a new archetype with the given name.
    pub fn archetype_name(
        self,
        name: impl Into<SorceryArchetypeName>,
    ) -> TerrestrialSorceryArchetypeBuilder {
        TerrestrialSorceryArchetypeBuilder::name(name)
    }

    /// Sets the archetype the sorcerer intiates into.
    pub fn add_archetype(
        self,
        archetype: AddSorceryArchetype,
    ) -> TerrestrialSorceryBuilderWithArchetype {
        TerrestrialSorceryBuilderWithArchetype {
            archetype_name: archetype.name,
            archetype: archetype.archetype,
        }
    }
}
