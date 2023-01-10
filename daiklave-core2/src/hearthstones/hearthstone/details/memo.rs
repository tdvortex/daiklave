use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, hearthstones::{category::HearthstoneCategory, geomancy_level::GeomancyLevel}};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HearthstoneDetailsMemo {
    name: String,
    book_reference: Option<BookReference>,
    category: HearthstoneCategory,
    geomancy_level: GeomancyLevel,
    lore: Option<String>,
    powers: Option<String>,
    is_dependent: bool,
}