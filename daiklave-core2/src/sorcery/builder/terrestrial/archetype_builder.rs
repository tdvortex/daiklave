use crate::{
    book_reference::BookReference,
    sorcery::{SorceryArchetypeDetails, SorceryArchetypeName},
};

use super::TerrestrialSorceryBuilderWithArchetype;

/// A builder to construct a new sorcery archetype to intiate a character into 
/// the Terrestrial Circle.
pub struct TerrestrialSorceryArchetypeBuilder {
    name: SorceryArchetypeName,
    book_reference: Option<BookReference>,
}

impl TerrestrialSorceryArchetypeBuilder {
    /// Starts the builder by supplying the archetype's name.
    pub fn name(name: impl Into<SorceryArchetypeName>) -> Self {
        Self {
            name: name.into(),
            book_reference: None,
        }
    }

    /// Sets the book reference for the archetype.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Supplies the description for the archetype.
    pub fn description(
        self,
        description: impl Into<String>,
    ) -> TerrestrialSorceryBuilderWithArchetype {
        TerrestrialSorceryBuilderWithArchetype {
            archetype_name: self.name,
            archetype: SorceryArchetypeDetails {
                book_reference: self.book_reference,
                description: description.into(),
            },
        }
    }
}
