use crate::{
    book_reference::BookReference,
    merits::merit::{
        instance::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName},
        AddSorceryArchetypeMerit,
    },
    sorcery::SorceryArchetypeName,
};

/// A sorcery archetype merit builder, after the description has been supplied.
pub struct SorceryArchetypeMeritBuilderWithDescription {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) name: SorceryArchetypeMeritName,
    pub(crate) description: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) dots: u8,
}

impl SorceryArchetypeMeritBuilderWithDescription {
    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Completes the builder, returning an AddSorceryArchetypeMerit.
    pub fn build(self) -> AddSorceryArchetypeMerit {
        AddSorceryArchetypeMerit {
            archetype_name: self.archetype_name,
            name: self.name,
            details: SorceryArchetypeMeritDetails {
                book_reference: self.book_reference,
                description: self.description,
                dots: self.dots,
            },
        }
    }
}
