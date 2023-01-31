use crate::book_reference::BookReference;

pub(crate) struct SorceryArchetypeMeritDetails {
    pub book_reference: Option<BookReference>,
    pub description: String,
    pub dots: u8,
}
