use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{category::HearthstoneCategory, geomancy_level::GeomancyLevel},
};

use super::HearthstoneDetails;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HearthstoneDetailsMemo {
    pub book_reference: Option<BookReference>,
    pub category: HearthstoneCategory,
    pub geomancy_level: GeomancyLevel,
    pub powers: String,
    pub is_dependent: bool,
}

impl From<&HearthstoneDetails<'_>> for HearthstoneDetailsMemo {
    fn from(view: &HearthstoneDetails<'_>) -> Self {
        Self {
            book_reference: view.book_reference,
            category: view.category,
            geomancy_level: view.geomancy_level,
            powers: view.powers.to_owned(),
            is_dependent: view.is_dependent,
        }
    }
}
