use crate::{
    book_reference::BookReference,
    hearthstones::hearthstone::{
        details::HearthstoneDetailsMemo, stability::HearthstoneStability,
        template::HearthstoneTemplate, AddHearthstone, GeomancyLevel, HearthstoneCategory,
    },
};

/// A hearthstone that is neither Linked nor WildBorn. It may be Manse-Born,
/// Steady, or Dependent, in any combination or none of the above.
pub struct NotLinkedOrWildbornHearthstoneBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) powers: String,
    pub(crate) category: HearthstoneCategory,
    pub(crate) geomancy_level: GeomancyLevel,
    pub(crate) is_manseborn: bool,
    pub(crate) is_steady: bool,
    pub(crate) is_dependent: bool,
}

impl NotLinkedOrWildbornHearthstoneBuilder {
    /// The book reference for the hearthstone.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds the Manse-Born keyword.
    pub fn manseborn(mut self) -> Self {
        self.is_manseborn = true;
        self
    }

    /// Adds the Steady keyword.
    pub fn steady(mut self) -> Self {
        self.is_steady = true;
        self
    }

    /// Adds the Dependent keyword.
    pub fn dependent(mut self) -> Self {
        self.is_dependent = true;
        self
    }

    /// Completes the builder, returning a hearthstone template and its name.
    pub fn build(self) -> AddHearthstone {
        let stability = match (self.is_manseborn, self.is_steady) {
            (true, true) => HearthstoneStability::ManseBornSteady,
            (true, false) => HearthstoneStability::ManseBorn,
            (false, true) => HearthstoneStability::Steady,
            (false, false) => HearthstoneStability::Unspecified,
        };

        (
            self.name,
            HearthstoneTemplate {
                details: HearthstoneDetailsMemo {
                    book_reference: self.book_reference,
                    category: self.category,
                    geomancy_level: self.geomancy_level,
                    powers: self.powers,
                    is_dependent: self.is_dependent,
                },
                stability,
            },
        )
    }
}
