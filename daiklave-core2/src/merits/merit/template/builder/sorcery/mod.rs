mod with_description;
mod with_dots;
mod with_name;
pub use with_description::SorceryArchetypeMeritBuilderWithDescription;
pub use with_dots::SorceryArchetypeMeritBuilderWithDots;
pub use with_name::SorceryArchetypeMeritBuilderWithName;

use crate::{sorcery::SorceryArchetypeName, book_reference::BookReference, merits::merit::instance::SorceryArchetypeMeritName};

/// A builder to construct a sorcery archetype merit.
pub struct SorceryArchetypeMeritBuilder {
    archetype_name: SorceryArchetypeName,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilder {
    /// Starts the builder by providing the name of the sorcery archetype the
    /// merit belongs to.
    pub fn archetype_name(archetype_name: impl Into<SorceryArchetypeName>) -> Self {
        Self {
            archetype_name: archetype_name.into(),
            book_reference: None,
        }
    }

    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the name for the merit.
    pub fn name(self, name: impl Into<SorceryArchetypeMeritName>) -> SorceryArchetypeMeritBuilderWithName {
        SorceryArchetypeMeritBuilderWithName {
            archetype_name: self.archetype_name,
            name: name.into(),
            book_reference: self.book_reference,
        }
    }
}