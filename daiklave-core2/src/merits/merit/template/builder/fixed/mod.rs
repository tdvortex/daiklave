mod nonstackable;
mod stackable;
mod with_description;
mod with_merit_type;
pub use nonstackable::FixedNonStackableMeritTemplateBuilder;
pub use stackable::FixedStackableMeritTemplateBuilder;
pub use with_description::FixedMeritTemplateBuilderWithDescription;
pub use with_merit_type::FixedMeritTemplateBuilderWithMeritType;

use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    merits::merit::{MeritPrerequisite, MeritType},
};

/// A builder for a merit with a fixed dot cost.
pub struct FixedMeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) dots: u8,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) book_reference: Option<BookReference>,
}

impl FixedMeritTemplateBuilder {
    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds a prerequisite to purchase the merit. Merit prerequisites are
    /// always and "or" relationship, like Stamina 3 or Resistance 3.
    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    /// Defines the type of the merit (like Innate or Purchased).
    pub fn merit_type(self, merit_type: MeritType) -> FixedMeritTemplateBuilderWithMeritType {
        FixedMeritTemplateBuilderWithMeritType {
            name: self.name,
            dots: self.dots,
            merit_type,
            prerequisites: self.prerequisites,
            book_reference: self.book_reference,
        }
    }
}
