mod category;
mod geomancy_level;
mod id;
mod keyword;
mod memo;
mod owned;

pub use id::HearthstoneId;
pub use owned::OwnedHearthstone;
pub(crate) use owned::OwnedHearthstoneMemo;

use std::collections::HashSet;

use crate::book_reference::BookReference;

use self::{
    category::HearthstoneCategory, geomancy_level::GeomancyLevel, keyword::HearthstoneKeyword,
    memo::HearthstoneMemo,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Hearthstone<'source> {
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
        HearthstoneMemo {
            name: self.name.to_string(),
            book_reference: self.book_reference,
            geomancy_level: self.geomancy_level,
            category: self.category,
            keywords: self.keywords.to_owned(),
            lore: self.lore.map(|s| s.to_string()),
            powers: self.powers.map(|s| s.to_string()),
        }
    }
}
