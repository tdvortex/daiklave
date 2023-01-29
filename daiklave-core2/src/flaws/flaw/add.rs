use crate::{book_reference::BookReference, CharacterMutation};

/// A Flaw to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFlaw {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub description: String,
}

impl From<AddFlaw> for CharacterMutation {
    fn from(add_flaw: AddFlaw) -> Self {
        Self::AddFlaw(add_flaw)
    }
}