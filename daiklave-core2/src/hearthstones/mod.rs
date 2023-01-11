/// Structs related to an individual hearthstone.
pub mod hearthstone;

use crate::Character;

pub use hearthstone::HearthstoneId;
pub(crate) use hearthstone::{
    HearthstonePosition, SlottedHearthstone, SlottedHearthstoneMemo, UnslottedHearthstone,
    UnslottedHearthstoneMemo,
};

use self::hearthstone::Hearthstone;

/// The Hearthstones owned by a character, their current position, and any
/// Manses and Demenses they may also have.
pub struct Hearthstones<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Hearthstones<'view, 'source> {
    /// Gets the details of a specific hearthstone by its Id.
    pub fn get(&self, _hearthstone_id: HearthstoneId) -> Option<Hearthstone<'source>> {
        todo!()
    }

    /// Iterates over all hearthstones owned by the character by their Ids.
    pub fn iter(&self) -> impl Iterator<Item = HearthstoneId> {
        vec![].into_iter()
    }
}
