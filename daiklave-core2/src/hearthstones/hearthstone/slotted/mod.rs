use crate::book_reference::BookReference;

use super::{
    category::HearthstoneCategory, details::HearthstoneDetails, geomancy_level::GeomancyLevel,
    keyword::HearthstoneKeyword, origin::HearthstoneOrigin,
};

mod memo;
pub(crate) use memo::SlottedHearthstoneMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SlottedHearthstone<'source> {
    pub name: &'source str,
    pub details: HearthstoneDetails<'source>,
    pub origin: HearthstoneOrigin<'source>,
}

impl<'source> From<&'source SlottedHearthstoneMemo> for SlottedHearthstone<'source> {
    fn from(memo: &'source SlottedHearthstoneMemo) -> Self {
        Self {
            name: memo.name.as_str(),
            details: (&memo.details).into(),
            origin: (&memo.origin).into(),
        }
    }
}

impl<'source> SlottedHearthstone<'source> {


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
