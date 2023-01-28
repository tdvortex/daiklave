use crate::{book_reference::BookReference, CharacterMutation};

/// A Flaw to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFlaw {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) description: String,
}

impl AddFlaw {
    /// Creates a new Flaw to be added to a character.
    pub fn new(name: String, book_reference: Option<BookReference>, description: String) -> Self {
        AddFlaw {
            name,
            book_reference,
            description,
        }
    }
}

impl From<AddFlaw> for CharacterMutation {
    fn from(add_flaw: AddFlaw) -> Self {
        Self::AddFlaw(add_flaw)
    }
}