use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Book {
    CoreRulebook,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct BookReference {
    book: Book,
    page_number: i16,
}

impl BookReference {
    pub fn new(book: Book, page_number: i16) -> Self {
        Self {
            book,
            page_number,
        }
    }
}