mod nonstackable;
mod stackable;
mod with_description;
mod with_dots;
mod with_merit_type;
pub use nonstackable::VariableNonStackableMeritTemplateBuilder;
pub use stackable::VariableStackableMeritTemplateBuilder;
pub use with_description::VariableMeritTemplateBuilderWithDescription;
pub use with_dots::VariableMeritTemplateBuilderWithDots;
pub use with_merit_type::VariableMeritTemplateBuilderWithMeritType;

use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    merits::merit::{MeritPrerequisite, MeritType},
};

/// A builder for a merit which can have multiple dot levels.
pub struct VariableMeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilder {
    /// Starts a new builder by providing the name of the merit.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            prerequisites: Default::default(),
            book_reference: Default::default(),
        }
    }

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

    /// Sets the type of the merit (like Innate or Purchased).
    pub fn merit_type(self, merit_type: MeritType) -> VariableMeritTemplateBuilderWithMeritType {
        VariableMeritTemplateBuilderWithMeritType {
            name: self.name,
            merit_type,
            book_reference: self.book_reference,
            prerequisites: self.prerequisites,
        }
    }
}
