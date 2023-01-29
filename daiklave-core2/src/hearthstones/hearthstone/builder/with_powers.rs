use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{
        details::HearthstoneDetailsMemo, stability::HearthstoneStability,
        template::HearthstoneTemplate, AddHearthstone, GeomancyLevel, HearthstoneCategory, HearthstoneName,
    },
};

use super::{
    LinkedHearthstoneBuilder, NotLinkedOrWildbornHearthstoneBuilder, WildBornHearthstoneBuilder,
};

/// A Hearthstone builder after its powers have been specified. If the
/// hearthstone has no keywords, it can be completed here with build().
pub struct HearthstoneBuilderWithPowers {
    pub(crate) name: HearthstoneName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) powers: String,
    pub(crate) category: HearthstoneCategory,
    pub(crate) geomancy_level: GeomancyLevel,
}

impl HearthstoneBuilderWithPowers {
    /// The book reference for the hearthstone.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds the Linked keyword.
    pub fn linked(self) -> LinkedHearthstoneBuilder {
        LinkedHearthstoneBuilder {
            name: self.name,
            book_reference: self.book_reference,
            powers: self.powers,
            category: self.category,
            geomancy_level: self.geomancy_level,
            is_dependent: false,
        }
    }

    /// Adds the Wildborn keyword.
    pub fn wildborn(self) -> WildBornHearthstoneBuilder {
        WildBornHearthstoneBuilder {
            name: self.name,
            book_reference: self.book_reference,
            powers: self.powers,
            category: self.category,
            geomancy_level: self.geomancy_level,
            is_dependent: false,
        }
    }

    /// Adds the Manse-Born keyword.
    pub fn manseborn(self) -> NotLinkedOrWildbornHearthstoneBuilder {
        NotLinkedOrWildbornHearthstoneBuilder {
            name: self.name,
            book_reference: self.book_reference,
            powers: self.powers,
            category: self.category,
            geomancy_level: self.geomancy_level,
            is_manseborn: true,
            is_steady: false,
            is_dependent: false,
        }
    }

    /// Adds the Steady keyword.
    pub fn steady(self) -> NotLinkedOrWildbornHearthstoneBuilder {
        NotLinkedOrWildbornHearthstoneBuilder {
            name: self.name,
            book_reference: self.book_reference,
            powers: self.powers,
            category: self.category,
            geomancy_level: self.geomancy_level,
            is_manseborn: false,
            is_steady: true,
            is_dependent: false,
        }
    }

    /// Adds the Dependent keyword.
    pub fn dependent(self) -> NotLinkedOrWildbornHearthstoneBuilder {
        NotLinkedOrWildbornHearthstoneBuilder {
            name: self.name,
            book_reference: self.book_reference,
            powers: self.powers,
            category: self.category,
            geomancy_level: self.geomancy_level,
            is_manseborn: false,
            is_steady: false,
            is_dependent: true,
        }
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
                    is_dependent: false,
                },
                stability: HearthstoneStability::Unspecified,
            },
        }
    }
}
