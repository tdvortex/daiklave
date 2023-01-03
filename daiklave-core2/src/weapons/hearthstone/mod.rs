use std::collections::HashSet;

use crate::book_reference::BookReference;

use self::{geomancy_level::GeomancyLevel, category::HearthstoneCategory, keyword::HearthstoneKeyword};

mod category;
mod geomancy_level;
mod keyword;
mod owned;


pub(in crate::weapons) use owned::{OwnedHearthstone, OwnedHearthstoneMemo};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::weapons) struct Hearthstone<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    geomancy_level: GeomancyLevel,
    category: HearthstoneCategory,
    keywords: &'source HashSet<HearthstoneKeyword>,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
}

impl<'source> Hearthstone<'source> {
    pub fn as_memo(&self) -> HearthstoneMemo {
        HearthstoneMemo { name: self.name.to_string(), 
            book_reference: self.book_reference, 
            geomancy_level: self.geomancy_level, 
            category: self.category, 
            keywords: self.keywords.to_owned(), 
            lore: self.lore.map(|s| s.to_string()), 
            powers: self.powers.map(|s| s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) struct HearthstoneMemo {
    name: String,
    book_reference: Option<BookReference>,
    geomancy_level: GeomancyLevel,
    category: HearthstoneCategory,
    keywords: HashSet<HearthstoneKeyword>,
    lore: Option<String>,
    powers: Option<String>,
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