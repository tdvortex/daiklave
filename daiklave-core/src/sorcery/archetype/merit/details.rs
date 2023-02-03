use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SorceryArchetypeMeritDetails {
    pub book_reference: Option<BookReference>,
    pub dots: u8,
    pub description: String,
}