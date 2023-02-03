use crate::{
    book_reference::BookReference,
    sorcery::{ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetypeName},
};

use super::TerrestrialSorceryBuilderWithShapingRitual;

/// A builder to construct a new shaping ritual to add to a character as part 
/// of their initiation into the Terrestrial circle of sorcery.
pub struct TerrestrialShapingRitualBuilder {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
    pub(crate) summary: String,
    pub(crate) book_reference: Option<BookReference>,
}

impl TerrestrialShapingRitualBuilder {
    /// Sets a book reference for the shaping ritual
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets a description for the ritual and completes the builder.
    pub fn description(
        self,
        description: impl Into<String>,
    ) -> TerrestrialSorceryBuilderWithShapingRitual {
        TerrestrialSorceryBuilderWithShapingRitual {
            archetype_name: self.archetype_name,
            archetype: self.archetype,
            shaping_ritual_summary: self.summary,
            shaping_ritual: ShapingRitualDetails {
                book_reference: self.book_reference,
                description: description.into(),
            },
        }
    }
}
