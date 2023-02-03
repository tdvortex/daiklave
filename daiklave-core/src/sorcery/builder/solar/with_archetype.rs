use crate::sorcery::{
    archetype::{AddShapingRitual, SorceryArchetypeName},
    SorceryArchetypeDetails, SorceryError,
};

use super::with_shaping_ritual::SolarSorceryBuilderWithShapingRitual;

/// A Solar sorcery builder after the archetype has been specified.
pub struct SolarSorceryBuilderWithArchetype {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetypeDetails>,
}

impl SolarSorceryBuilderWithArchetype {
    /// Sets the shaping ritual for this circle.
    pub fn shaping_ritual(
        self,
        shaping_ritual: AddShapingRitual,
    ) -> Result<SolarSorceryBuilderWithShapingRitual, SorceryError> {
        if shaping_ritual.archetype_name != self.archetype_name {
            Err(SorceryError::MissingArchetype)
        } else {
            Ok(SolarSorceryBuilderWithShapingRitual {
                archetype_name: self.archetype_name,
                archetype: self.archetype,
                shaping_ritual_summary: shaping_ritual.summary,
                shaping_ritual: shaping_ritual.ritual,
            })
        }
    }
}
