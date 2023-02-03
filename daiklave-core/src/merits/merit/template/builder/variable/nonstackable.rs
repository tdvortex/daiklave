use crate::{
    book_reference::BookReference,
    merits::merit::{template::nonstackable::VariableNonStackableMeritTemplate, MeritPrerequisite},
};

use super::VariableMeritTemplateBuilderWithDots;

/// A builder for a merit which can be bought at multiple levels, but only once
/// per character.
pub struct VariableNonStackableMeritTemplateBuilder(
    pub(crate) VariableMeritTemplateBuilderWithDots,
);

impl VariableNonStackableMeritTemplateBuilder {
    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self = Self(self.0.book_reference(book_reference));
        self
    }

    /// Adds a prerequisite to purchase the merit. Merit prerequisites are
    /// always and "or" relationship, like Stamina 3 or Resistance 3.
    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self = Self(self.0.prerequisite(prerequisite));
        self
    }

    /// Adds a dot level at which the merit can be purchased. These need not be
    /// consecutive.
    pub fn dot_option(mut self, dots: u8, description: impl Into<String>) -> Self {
        self = Self(self.0.dot_option(dots, description));
        self
    }

    /// Completes the builder, returning a merit template.
    pub fn build(self) -> VariableNonStackableMeritTemplate {
        VariableNonStackableMeritTemplate {
            name: self.0.name.into(),
            book_reference: self.0.book_reference,
            merit_type: self.0.merit_type,
            description: self.0.description,
            prerequisites: self.0.prerequisites,
            min_dots: self.0.min_dots,
            other_dots: self.0.other_dots,
        }
    }
}
