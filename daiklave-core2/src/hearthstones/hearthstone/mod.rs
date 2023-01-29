use crate::{artifact::ArtifactName, book_reference::BookReference};

mod add;
/// A builder path for creating new Hearthstones.
pub mod builder;
mod category;
mod details;
mod geomancy_level;
mod keyword;
mod name;
mod origin;
mod position;
mod slot;
mod slotted;
mod stability;
mod template;
mod unslot;
mod unslotted;

pub use add::{AddHearthstone};
pub use name::HearthstoneName;
pub use slot::SlotHearthstone;
pub use unslot::UnslotHearthstone;
pub(crate) use origin::HearthstoneOrigin;
pub(crate) use position::HearthstonePosition;
pub(crate) use slotted::{SlottedHearthstone, SlottedHearthstoneMemo};
pub(crate) use stability::HearthstoneStability;
pub(crate) use unslotted::{UnslottedHearthstone, UnslottedHearthstoneMemo};

use self::builder::HearthstoneBuilder;
pub use {
    category::HearthstoneCategory, geomancy_level::GeomancyLevel, keyword::HearthstoneKeyword,
    template::HearthstoneTemplate,
};

/// A Hearthstone owned by a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hearthstone<'source>(pub(crate) HearthstonePosition<'source>);

impl<'source> Hearthstone<'source> {
    /// Starts constructing a new HearthstoneTemplate with a name.
    pub fn builder(name: String) -> HearthstoneBuilder {
        HearthstoneBuilder {
            name,
            book_reference: None,
        }
    }

    /// The name of the hearthstone.
    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    /// If the hearthstone is currently slotted into an artifact, the Id of
    /// that artifact.
    pub fn slotted_into(&self) -> Option<ArtifactName<'source>> {
        self.0.slotted_into()
    }

    /// The book reference for the hearthstone, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// The category of the hearthstone.
    pub fn category(&self) -> HearthstoneCategory {
        self.0.category()
    }

    /// The powers granted by the hearthstone, in addition to the basic mote
    /// regeneration benefits.
    pub fn powers(&self) -> &'source str {
        self.0.powers()
    }

    /// The level of the hearthstone, Standard or Greater.
    pub fn geomancy_level(&self) -> GeomancyLevel {
        self.0.geomancy_level()
    }

    /// An iterator over the hearthstone's keywords in alphabetical order.
    pub fn keywords(&self) -> impl Iterator<Item = HearthstoneKeyword> {
        self.0.keywords()
    }

    /// The name of the manse and that manse's domain, if they exist.
    pub fn manse_and_demense(&self) -> Option<(&'source str, &'source str)> {
        self.0.manse_and_demense()
    }
}
