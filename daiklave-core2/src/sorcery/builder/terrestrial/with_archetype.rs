use crate::sorcery::{
    archetype::{AddShapingRitual, SorceryArchetypeName},
    SorceryArchetype, SorceryError,
};

use super::TerrestrialSorceryBuilderWithShapingRitual;

/// A Terrestrial Sorcery builder after the archetype has been specified.
pub struct TerrestrialSorceryBuilderWithArchetype {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetype,
}

impl TerrestrialSorceryBuilderWithArchetype {
    /// Sets the shaping ritual for the circle.
    pub fn shaping_ritual(
        self,
        shaping_ritual: AddShapingRitual,
    ) -> Result<TerrestrialSorceryBuilderWithShapingRitual, SorceryError> {
        if self.archetype_name != shaping_ritual.archetype_name {
            Err(SorceryError::MissingArchetype)
        } else {
            Ok(TerrestrialSorceryBuilderWithShapingRitual {
                archetype_name: self.archetype_name,
                archetype: self.archetype,
                shaping_ritual_summary: shaping_ritual.summary,
                shaping_ritual: shaping_ritual.ritual,
            })
        }
    }
}
