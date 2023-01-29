use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ShapingRitualDetails {
    pub book_reference: Option<BookReference>,
    pub description: String,
}