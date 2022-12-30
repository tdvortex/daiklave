#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Book {
    CoreRulebook,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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