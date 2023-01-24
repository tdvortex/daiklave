use crate::book_reference::BookReference;

/// A Flaw to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlawMutation {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) description: String,
}

impl FlawMutation {
    /// Creates a new Flaw to be added to a character.
    pub fn new(name: String, book_reference: Option<BookReference>, description: String) -> Self {
        FlawMutation {
            name,
            book_reference,
            description,
        }
    }
}