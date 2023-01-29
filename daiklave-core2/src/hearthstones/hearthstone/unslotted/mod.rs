mod memo;
pub(crate) use memo::UnslottedHearthstoneMemo;

use crate::book_reference::BookReference;

use super::{
    category::HearthstoneCategory, details::HearthstoneDetails, geomancy_level::GeomancyLevel,
    keyword::HearthstoneKeyword, origin::HearthstoneOrigin,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnslottedHearthstone<'source> {
    pub(crate) details: HearthstoneDetails<'source>,
    pub(crate) origin: HearthstoneOrigin<'source>,
}

impl<'source> UnslottedHearthstone<'source> {
    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference()
    }

    pub fn category(&self) -> HearthstoneCategory {
        self.details.category()
    }

    pub fn powers(&self) -> &'source str {
        self.details.powers()
    }

    pub fn geomancy_level(&self) -> GeomancyLevel {
        self.details.geomancy_level()
    }

    pub fn keywords(&self) -> impl Iterator<Item = HearthstoneKeyword> {
        self.origin.keywords().chain(self.details.keywords())
    }

    pub fn manse_and_demense(&self) -> Option<(&'source str, &'source str)> {
        self.origin.manse_and_demense()
    }
}
