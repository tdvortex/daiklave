use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{
        details::HearthstoneDetailsMemo, stability::HearthstoneStability,
        template::HearthstoneTemplate, AddHearthstone, GeomancyLevel, HearthstoneCategory,
        HearthstoneName,
    },
};

/// A Wild-Born hearthstone. It may not be Linked, ManseBorn, or Steady, but it
/// may be Dependent.
pub struct WildBornHearthstoneBuilder {
    pub(crate) name: HearthstoneName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) powers: String,
    pub(crate) category: HearthstoneCategory,
    pub(crate) geomancy_level: GeomancyLevel,
    pub(crate) is_dependent: bool,
}

impl WildBornHearthstoneBuilder {
    /// The book reference for the hearthstone.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds the Dependent keyword.
    pub fn dependent(mut self) -> Self {
        self.is_dependent = true;
        self
    }

    /// Completes the builder, returning an AddHearthstone struct.
    pub fn build(self) -> AddHearthstone {
        AddHearthstone {
            name: self.name,
            template: HearthstoneTemplate {
                details: HearthstoneDetailsMemo {
                    book_reference: self.book_reference,
                    category: self.category,
                    geomancy_level: self.geomancy_level,
                    powers: self.powers,
                    is_dependent: self.is_dependent,
                },
                stability: HearthstoneStability::WildBorn,
            },
        }
    }
}
