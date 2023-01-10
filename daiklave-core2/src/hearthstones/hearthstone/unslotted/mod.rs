use crate::{book_reference::BookReference, hearthstones::{category::HearthstoneCategory, geomancy_level::GeomancyLevel, keyword::HearthstoneKeyword}};

use super::{details::HearthstoneDetails, origin::HearthstoneOrigin};

pub(crate) struct UnslottedHearthstone<'source> {
    details: HearthstoneDetails<'source>,
    origin: HearthstoneOrigin<'source>,
}

impl<'source> UnslottedHearthstone<'source> {
    pub fn name(&self) -> &'source str {
        self.details.name()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference()
    }

    pub fn category(&self) -> HearthstoneCategory {
        self.details.category()
    }

    pub fn lore(&self) -> Option<&'source str> {
        self.details.lore()
    }

    pub fn powers(&self) -> Option<&'source str> {
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