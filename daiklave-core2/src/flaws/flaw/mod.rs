mod mutation;
pub use mutation::FlawMutation;

use crate::book_reference::BookReference;

/// A Flaw belonging to a character.
pub struct Flaw<'source> {
    pub(crate) name: &'source str,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) description: &'source str,
}

impl<'source> Flaw<'source> {
    /// The name of the flaw.
    pub fn name(&self) -> &'source str {
        self.name
    }

    /// The book reference for the Flaw, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// A description of the Flaw's mechanical effects.
    pub fn description(&self) -> &'source str {
        self.description
    }
}