use crate::{sorcery::SorceryArchetypeName, book_reference::BookReference, merits::merit::instance::SorceryArchetypeMeritName};

use super::SorceryArchetypeMeritBuilderWithDescription;

pub struct SorceryArchetypeMeritBuilderWithDots {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) name: SorceryArchetypeMeritName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) dots: u8,
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
