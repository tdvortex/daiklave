use serde::{Deserialize, Serialize};

use super::Book;

/// A reference to a specific page of a specific Exalted 3e book.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct BookReference {
    book: Book,
    page_number: i16,
}

impl BookReference {
    /// Constructs a new BookReference.
    pub fn new(book: Book, page_number: i16) -> Self {
        Self { book, page_number }
    }
}
