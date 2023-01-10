use crate::{artifact::ArtifactId, book_reference::BookReference};

mod category;
mod details;
mod geomancy_level;
mod id;
mod keyword;
mod origin;
mod position;
mod slotted;
mod stability;
mod template;
mod unslotted;

pub(crate) use position::HearthstonePosition;
pub(crate) use slotted::{SlottedHearthstone, SlottedHearthstoneMemo};
pub use {
    category::HearthstoneCategory, geomancy_level::GeomancyLevel, id::HearthstoneId,
    keyword::HearthstoneKeyword,
};

/// A Hearthstone owned by a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hearthstone<'source>(pub(crate) HearthstonePosition<'source>);

impl<'source> Hearthstone<'source> {
    pub fn id(&self) -> HearthstoneId {
        self.0.id()
    }

    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    pub fn slotted_into(&self) -> Option<ArtifactId> {
        self.0.slotted_into()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    pub fn category(&self) -> HearthstoneCategory {
        self.0.category()
    }

    pub fn lore(&self) -> Option<&'source str> {
        self.0.lore()
    }

    pub fn powers(&self) -> Option<&'source str> {
        self.0.powers()
    }

    pub fn geomancy_level(&self) -> GeomancyLevel {
        self.0.geomancy_level()
    }

    pub fn keywords(&self) -> impl Iterator<Item = HearthstoneKeyword> {
        self.0.keywords()
    }

    pub fn manse_and_demense(&self) -> Option<(&'source str, &'source str)> {
        self.0.manse_and_demense()
    }
}
