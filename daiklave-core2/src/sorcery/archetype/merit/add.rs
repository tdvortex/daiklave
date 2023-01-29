use crate::{sorcery::SorceryArchetypeName, book_reference::BookReference};

use super::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName};

pub struct AddSorceryArchetypeMerit {
    archetype_name: SorceryArchetypeName,
    merit_name: SorceryArchetypeMeritName,
    merit: SorceryArchetypeMeritDetails,
}

impl AddSorceryArchetypeMerit {
    pub fn new(
        archetype_name: SorceryArchetypeName,
        merit_name: SorceryArchetypeMeritName,
        book_reference: Option<BookReference>,
        dots: u8,
        description: String,
    ) -> AddSorceryArchetypeMerit {
        AddSorceryArchetypeMerit {
            archetype_name,
            merit_name,
            merit: SorceryArchetypeMeritDetails {
                book_reference,
                dots,
                description,
            }
        }
    }
}