use crate::book_reference::BookReference;

use super::{
    category::HearthstoneCategory, details::HearthstoneDetails, geomancy_level::GeomancyLevel,
    id::HearthstoneId, keyword::HearthstoneKeyword, origin::HearthstoneOrigin,
};

mod memo;
pub(crate) use memo::SlottedHearthstoneMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SlottedHearthstone<'source> {
    hearthstone_id: HearthstoneId,
    details: HearthstoneDetails<'source>,
    origin: HearthstoneOrigin<'source>,
}

impl<'source> SlottedHearthstone<'source> {
    pub fn as_memo(&self) -> SlottedHearthstoneMemo {
        SlottedHearthstoneMemo {
            hearthstone_id: self.hearthstone_id,
            details: self.details.as_memo(),
            origin: self.origin.as_memo(),
        }
    }

    pub fn id(&self) -> HearthstoneId {
        self.hearthstone_id
    }

    pub fn name(&self) -> &'source str {
        self.details.name()
    }

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