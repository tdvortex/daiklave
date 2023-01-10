use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{hearthstone::{GeomancyLevel, HearthstoneCategory, HearthstoneKeyword}, Hearthstone};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HearthstoneMemo {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub geomancy_level: GeomancyLevel,
    pub category: HearthstoneCategory,
    pub keywords: HashSet<HearthstoneKeyword>,
    pub lore: Option<String>,
    pub powers: Option<String>,
}

impl<'source> HearthstoneMemo {
    pub fn as_ref(&'source self) -> Hearthstone<'source> {
        Hearthstone {
            name: self.name.as_str(),
            book_reference: self.book_reference,
            geomancy_level: self.geomancy_level,
            category: self.category,
            keywords: &self.keywords,
            lore: self.lore.as_deref(),
            powers: self.powers.as_deref(),
        }
    }
}
