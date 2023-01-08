mod book;

pub use book::Book;
use serde::{Deserialize, Serialize};
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

    /// The book being referenced.
    pub fn book(&self) -> Book {
        self.book
    }

    /// The page number in the referenced book.
    pub fn page_number(&self) -> i16 {
        self.page_number
    }
}
