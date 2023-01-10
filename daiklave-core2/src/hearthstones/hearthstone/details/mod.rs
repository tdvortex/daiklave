mod memo;
pub(crate) use memo::HearthstoneDetailsMemo;

use crate::{book_reference::BookReference, hearthstones::{category::HearthstoneCategory, geomancy_level::GeomancyLevel, keyword::HearthstoneKeyword}};

pub(crate) struct HearthstoneDetails<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    category: HearthstoneCategory,
    geomancy_level: GeomancyLevel,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
    is_dependent: bool,
}

impl<'source> HearthstoneDetails<'source> {
    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn category(&self) -> HearthstoneCategory {
        self.category
    }

    pub fn lore(&self) -> Option<&'source str> {
        self.lore
    }

    pub fn powers(&self) -> Option<&'source str> {
        self.powers
    }

    pub fn geomancy_level(&self) -> GeomancyLevel {
        self.geomancy_level
    }

    pub fn keywords(&self) -> impl Iterator<Item = HearthstoneKeyword> {
        if self.is_dependent {
            vec![HearthstoneKeyword::Dependent]
        } else {
            vec![]
        }.into_iter()
    }
}