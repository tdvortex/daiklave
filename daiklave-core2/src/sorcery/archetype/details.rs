use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SorceryArchetypeDetails {
    pub book_reference: Option<BookReference>,
    pub description: String,
}
