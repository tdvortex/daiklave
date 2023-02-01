use crate::{book_reference::BookReference, merits::merit_new::{MeritPrerequisite, template::nonstackable::VariableNonStackableMeritTemplate}};

use super::VariableMeritTemplateBuilderWithDots;

pub struct VariableNonStackableMeritTemplateBuilder(pub(crate) VariableMeritTemplateBuilderWithDots);

impl VariableNonStackableMeritTemplateBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self = Self(self.0.book_reference(book_reference));
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self = Self(self.0.prerequisite(prerequisite));
        self
    }

    pub fn dot_option(mut self, dots: u8, description: impl Into<String>) -> Self {
        self = Self(self.0.dot_option(dots, description));
        self
    }

    pub fn build(self) -> VariableNonStackableMeritTemplate {
        VariableNonStackableMeritTemplate {
            name: self.0.name.into(),
            book_reference: self.0.book_reference,
            merit_type: self.0.merit_type,
            description: self.0.description,
            prerequisites: self.0.prerequisites,
            min_dots: self.0.min_dots,
            other_dots: self.0.other_dots
        }
    }
}