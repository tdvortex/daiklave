mod linked;
mod not_linked_wildborn;
mod wildborn;
mod with_category;
mod with_level;
mod with_powers;

pub use linked::LinkedHearthstoneBuilder;
pub use not_linked_wildborn::NotLinkedOrWildbornHearthstoneBuilder;
pub use wildborn::WildBornHearthstoneBuilder;
pub use with_category::HearthstoneBuilderWithCategory;
pub use with_level::HearthstoneBuilderWithLevel;
pub use with_powers::HearthstoneBuilderWithPowers;

use crate::book_reference::BookReference;

use super::{HearthstoneCategory, HearthstoneName};

/// A builder path to construct a Hearthstone. Required fields (in order) are
/// name, category, level, and powers. After powers, keywords may be provided.
/// If keywords are provided, they must be specified in the following order:
/// Linked, WildBorn, ManseBorn, Steady, Dependent. Linked hearthstones may be
/// Dependent, but cannot have any other keywords. Wildborn hearthstones may be
/// Dependent, but cannot have any other keywords. Hearthstones that are not
/// Linked and not WildBorn may have any combination of ManseBorn, Steady, or
/// Dependent, or none of them.
pub struct HearthstoneBuilder {
    pub(crate) name: HearthstoneName,
    pub(crate) book_reference: Option<BookReference>,
}

impl HearthstoneBuilder {
    /// Starts a new builder with the specified name.
    pub fn name(name: impl Into<HearthstoneName>) -> Self {
        Self {
            name: name.into(),
            book_reference: None,
        }
    }

    /// The book reference for the hearthstone.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the category for the hearthstone.
    pub fn category(self, category: HearthstoneCategory) -> HearthstoneBuilderWithCategory {
        HearthstoneBuilderWithCategory {
            name: self.name,
            book_reference: self.book_reference,
            category,
        }
    }
}
