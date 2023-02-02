use crate::{sorcery::SorceryArchetypeName, merits::merit::instance::SorceryArchetypeMeritName, book_reference::BookReference};

use super::SorceryArchetypeMeritBuilderWithDots;

pub struct SorceryArchetypeMeritBuilderWithName {
    archetype_name: SorceryArchetypeName,
    name: SorceryArchetypeMeritName,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilderWithName {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn dots(self, dots: u8) -> SorceryArchetypeMeritBuilderWithDots {
        SorceryArchetypeMeritBuilderWithDots {
            archetype_name: self.archetype_name,
            name: self.name,
            book_reference: self.book_reference,
            dots,
        }
    }
}