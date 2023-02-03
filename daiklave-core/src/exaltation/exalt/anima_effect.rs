use crate::book_reference::BookReference;

/// A description of an Anima Effect belonging to an Exalt.
pub struct AnimaEffect<'source> {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) description: &'source str,
}

impl<'source> AnimaEffect<'source> {
    /// The book reference for the anima effect, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The descripive text of the anima effect.
    pub fn description(&self) -> &'source str {
        self.description
    }
}
