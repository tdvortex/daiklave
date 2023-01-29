use crate::book_reference::BookReference;

use super::{SorceryArchetypeDetails, SorceryArchetypeName};

pub struct AddSorceryArchetype {
    pub(crate) name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails
}

impl AddSorceryArchetype {
    pub fn new(
        name: impl ToString,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            name: name.into(),
            archetype: SorceryArchetypeDetails {
                book_reference,
                description,
            },
        }
    }
}