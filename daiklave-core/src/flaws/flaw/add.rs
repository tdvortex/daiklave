use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, CharacterMutation};

use super::FlawName;

/// A Flaw to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddFlaw {
    /// The name of the Flaw.
    pub name: FlawName,
    /// The page reference for the Flaw, if any.
    pub book_reference: Option<BookReference>,
    /// The description of the Flaw.
    pub description: String,
}

impl From<AddFlaw> for CharacterMutation {
    fn from(add_flaw: AddFlaw) -> Self {
        Self::AddFlaw(add_flaw)
    }
}
