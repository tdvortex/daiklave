mod hearthstone;

pub use hearthstone::Hearthstone;

use crate::Character;

pub use hearthstone::HearthstoneId;
pub(crate) use hearthstone::{HearthstonePosition, SlottedHearthstone, SlottedHearthstoneMemo};

/// The Hearthstones owned by a character, their current position, and any
/// Manses and Demenses they may also have.
pub struct Hearthstones<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Hearthstones<'view, 'source> {
    pub fn get(&self, _hearthstone_id: HearthstoneId) -> Option<Hearthstone<'source>> {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = HearthstoneId> {
        vec![].into_iter()
    }
}
