use crate::{sorcery::SorceryArchetypeName, book_reference::BookReference, merits::merit_new::instance::SorceryArchetypeMeritName};

use super::SorceryArchetypeMeritBuilderWithDescription;

pub struct SorceryArchetypeMeritBuilderWithDots {
    archetype_name: SorceryArchetypeName,
    name: SorceryArchetypeMeritName,
    book_reference: Option<BookReference>,
    dots: u8,
}

impl SorceryArchetypeMeritBuilderWithDots {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn description(self, description: impl Into<String>) -> SorceryArchetypeMeritBuilderWithDescription {
        SorceryArchetypeMeritBuilderWithDescription {
            archetype_name: self.archetype_name,
            name: self.name,
            book_reference: self.book_reference,
            dots: self.dots,
            description: description.into(),
        }
    }
}
