use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{GeomancyLevel, HearthstoneCategory, HearthstoneName},
};

use super::HearthstoneBuilderWithLevel;

/// A Hearthstone builder after category has been specified.
pub struct HearthstoneBuilderWithCategory {
    pub(crate) name: HearthstoneName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) category: HearthstoneCategory,
}

impl HearthstoneBuilderWithCategory {
    /// The book reference for the hearthstone.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the hearthstone to be either Standard or Greater.
    pub fn geomancy_level(self, geomancy_level: GeomancyLevel) -> HearthstoneBuilderWithLevel {
        HearthstoneBuilderWithLevel {
            name: self.name,
            book_reference: self.book_reference,
            category: self.category,
            geomancy_level,
        }
    }
}
