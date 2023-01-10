use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{category::HearthstoneCategory, geomancy_level::GeomancyLevel},
};

use super::HearthstoneDetails;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HearthstoneDetailsMemo {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub category: HearthstoneCategory,
    pub geomancy_level: GeomancyLevel,
    pub lore: Option<String>,
    pub powers: Option<String>,
    pub is_dependent: bool,
}

impl<'source> HearthstoneDetailsMemo {
    pub fn as_ref(&'source self) -> HearthstoneDetails<'source> {
        HearthstoneDetails {
            name: self.name.as_str(),
            book_reference: self.book_reference,
            category: self.category,
            geomancy_level: self.geomancy_level,
            lore: self.lore.as_deref(),
            powers: self.powers.as_deref(),
            is_dependent: self.is_dependent,
        }
    }
}
