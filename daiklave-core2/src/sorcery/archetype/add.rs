use crate::book_reference::BookReference;

use super::{SorceryArchetype, SorceryArchetypeName};

pub struct AddSorceryArchetype {
    pub(crate) name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetype
}

impl AddSorceryArchetype {
    pub fn new(
        name: impl ToString,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            name: name.into(),
            archetype: SorceryArchetype {
                book_reference,
                description,
            },
        }
    }
}