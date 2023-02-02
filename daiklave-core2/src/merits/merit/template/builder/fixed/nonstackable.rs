use crate::{merits::merit::{template::nonstackable::FixedNonStackableMeritTemplate, MeritPrerequisite}, book_reference::BookReference};

use super::FixedMeritTemplateBuilderWithDescription;

/// A merit builder for a fixed-dot merit that may be only be purchased once.
pub struct FixedNonStackableMeritTemplateBuilder(pub(crate) FixedMeritTemplateBuilderWithDescription);

impl FixedNonStackableMeritTemplateBuilder {
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

    /// Completes the builder, returning a merit template.
    pub fn build(self) -> FixedNonStackableMeritTemplate {
        FixedNonStackableMeritTemplate {
            name: self.0.name.into(),
            book_reference: self.0.book_reference,
            merit_type: self.0.merit_type,
            description: self.0.description,
            prerequisites: self.0.prerequisites,
            dots: self.0.dots,
        }
    }
}