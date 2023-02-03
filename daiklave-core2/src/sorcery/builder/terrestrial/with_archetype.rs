use crate::sorcery::{
    archetype::{AddShapingRitual, SorceryArchetypeName},
    SorceryArchetypeDetails, SorceryError,
};

use super::{TerrestrialShapingRitualBuilder, TerrestrialSorceryBuilderWithShapingRitual};

/// A Terrestrial Sorcery builder after the archetype has been specified.
pub struct TerrestrialSorceryBuilderWithArchetype {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
}

impl TerrestrialSorceryBuilderWithArchetype {
    /// Starts building a new shaping ritual for the archetype by supplying 
    /// a short, unique summary for it.
    pub fn shaping_ritual_summary(
        self,
        summary: impl Into<String>,
    ) -> TerrestrialShapingRitualBuilder {
        TerrestrialShapingRitualBuilder {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            summary: summary.into(),
            book_reference: None,
        }
    }

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
