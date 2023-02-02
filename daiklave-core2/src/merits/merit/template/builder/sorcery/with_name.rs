use crate::{sorcery::SorceryArchetypeName, merits::merit::instance::SorceryArchetypeMeritName, book_reference::BookReference};

use super::SorceryArchetypeMeritBuilderWithDots;

/// A sorcery archetype merit builder after the name has been supplied.
pub struct SorceryArchetypeMeritBuilderWithName {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) name: SorceryArchetypeMeritName,
    pub(crate) book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilderWithName {
    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the dot level of the merit.
    pub fn dots(self, dots: u8) -> SorceryArchetypeMeritBuilderWithDots {
        SorceryArchetypeMeritBuilderWithDots {
            archetype_name: self.archetype_name,
            name: self.name,
            book_reference: self.book_reference,
            dots,
        }
    }
}