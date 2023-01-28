use crate::sorcery::{
    archetype::{AddShapingRitual, SorceryArchetypeName},
    SorceryArchetype, SorceryError,
};

use super::with_shaping_ritual::CelestialSorceryBuilderWithShapingRitual;

/// A Celestial Sorcery builder after the archetype has been specified.
pub struct CelestialSorceryBuilderWithArchetype {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: Option<SorceryArchetype>,
}

impl CelestialSorceryBuilderWithArchetype {
    /// Sets the shaping ritual for this circle.
    pub fn shaping_ritual(
        self,
        shaping_ritual: AddShapingRitual,
    ) -> Result<CelestialSorceryBuilderWithShapingRitual, SorceryError> {
        if shaping_ritual.0 != self.archetype_name {
            Err(SorceryError::MissingArchetype)
        } else {
            Ok(CelestialSorceryBuilderWithShapingRitual {
                archetype_name: self.archetype_name,
                archetype: self.archetype,
                shaping_ritual_name: shaping_ritual.1,
                shaping_ritual: shaping_ritual.2,
            })
        }
    }
}
