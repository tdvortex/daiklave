use crate::{book_reference::BookReference, merits::merit_new::{Merit, MeritSource}};

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
    pub(crate) fn merits(&self) -> Vec<Merit<'source>> {
        if let Some((manse, demense)) = self.manse_and_demense() {
            vec![
                Merit(MeritSource::Demense { name: demense, has_manse: true, geomancy_level: self.geomancy_level() }),
                Merit(MeritSource::Hearthstone { name: self.name, has_manse: true, geomancy_level: self.geomancy_level() }),
                Merit(MeritSource::Manse { name: manse, geomancy_level: self.geomancy_level() })
            ]
        } else {
            vec![
                Merit(MeritSource::Hearthstone { name: self.name, has_manse: false, geomancy_level: self.geomancy_level() })
            ]
        }
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
