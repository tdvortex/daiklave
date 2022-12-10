use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookReference {
    pub book_title: String,
    pub page_number: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSource {
    Custom(Option<i32>),
    Book(BookReference),
}

impl DataSource {
    pub(crate) fn book_title(&self) -> Option<&str> {
        match self {
            DataSource::Custom(_) => None,
            DataSource::Book(book_reference) => Some(book_reference.book_title.as_str()),
        }
    }

    pub(crate) fn page_number(&self) -> Option<i16> {
        match self {
            DataSource::Custom(_) => None,
            DataSource::Book(book_reference) => Some(book_reference.page_number),
        }
    }

    pub(crate) fn creator_id(&self) -> Option<i32> {
        match self {
            DataSource::Custom(creator_id) => *creator_id,
            DataSource::Book(_) => None,
        }
    }
}
