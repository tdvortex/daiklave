mod memo;
pub(crate) use memo::HearthstoneDetailsMemo;

use crate::book_reference::BookReference;

use super::{
    category::HearthstoneCategory, geomancy_level::GeomancyLevel, keyword::HearthstoneKeyword,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HearthstoneDetails<'source> {
    book_reference: Option<BookReference>,
    category: HearthstoneCategory,
    geomancy_level: GeomancyLevel,
    powers: &'source str,
    is_dependent: bool,
}

impl<'source> From<&'source HearthstoneDetailsMemo> for HearthstoneDetails<'source> {
    fn from(memo: &'source HearthstoneDetailsMemo) -> Self {
        Self {
            book_reference: memo.book_reference,
            category: memo.category,
            geomancy_level: memo.geomancy_level,
            powers: memo.powers.as_str(),
            is_dependent: memo.is_dependent,
        }
    }
}

impl<'source> HearthstoneDetails<'source> {
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn category(&self) -> HearthstoneCategory {
        self.category
    }

    pub fn powers(&self) -> &'source str {
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
        }
        .into_iter()
    }
}
