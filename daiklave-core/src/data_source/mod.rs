use serde::{Deserialize, Serialize};

use crate::id::CharacterId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BookReference {
    pub book_title: String,
    pub page_number: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSource {
    Custom(CharacterId),
    Book(BookReference),
}

impl DataSource {
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    pub fn book_title(&self) -> Option<&str> {
        match self {
            DataSource::Custom(_) => None,
            DataSource::Book(book_reference) => Some(book_reference.book_title.as_str()),
        }
    }

    pub fn page_number(&self) -> Option<i16> {
        match self {
            DataSource::Custom(_) => None,
            DataSource::Book(book_reference) => Some(book_reference.page_number),
        }
    }

    pub fn creator_id(&self) -> Option<CharacterId> {
        match self {
            DataSource::Custom(creator_id) => Some(*creator_id),
            DataSource::Book(_) => None,
        }
    }
}
