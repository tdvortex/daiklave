mod no_attunement;
pub(crate) use no_attunement::{WonderNoAttunement, WonderNoAttunementMemo};

use crate::{artifact::MagicMaterial, book_reference::BookReference};

use super::WonderId;

/// A Wonder that belongs to the character, and may be attuned or unattuned.
pub struct OwnedWonder<'source>(
    pub(crate) WonderId,
    pub(crate) WonderNoAttunement<'source>,
    pub(crate) Option<u8>,
);

impl<'source> OwnedWonder<'source> {
    /// The wonder's Id.
    pub fn id(&self) -> WonderId {
        self.0
    }

    /// The wonder's name.
    pub fn name(&self) -> &'source str {
        self.1.name()
    }

    /// The book reference for the wonder.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.1.book_reference()
    }

    /// The powers of the wonder.
    pub fn powers(&self) -> &'source str {
        self.1.powers()
    }

    /// The history and flavor text of the wonder, if any.
    pub fn lore(&self) -> Option<&'source str> {
        self.1.lore()
    }

    /// The total number of hearthstone slots on the wonder.
    pub fn hearthstone_slots(&self) -> u8 {
        self.1.hearthstone_slots()
    }

    /// Returns true if the wonder is currently attuned.
    pub fn is_attuned(&self) -> bool {
        self.2.is_some()
    }

    /// If the wonder has a specific Magic Material, returns it.
    pub fn material(&self) -> Option<MagicMaterial> {
        self.1.material()
    }
}
