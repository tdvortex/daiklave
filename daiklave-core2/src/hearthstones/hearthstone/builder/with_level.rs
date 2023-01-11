use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{GeomancyLevel, HearthstoneCategory},
};

use super::HearthstoneBuilderWithPowers;

/// A Hearthstone builder after its geomancy level has been specified.
pub struct HearthstoneBuilderWithLevel {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) category: HearthstoneCategory,
    pub(crate) geomancy_level: GeomancyLevel,
}

impl HearthstoneBuilderWithLevel {
    /// The book reference for the hearthstone.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the hearthstone's powers.
    pub fn powers(self, powers: String) -> HearthstoneBuilderWithPowers {
        HearthstoneBuilderWithPowers {
            name: self.name,
            book_reference: self.book_reference,
            powers,
            category: self.category,
            geomancy_level: self.geomancy_level,
        }
    }
}
