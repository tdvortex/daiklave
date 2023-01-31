use crate::{merits::merit_new::{template::stackable::VariableStackableMeritTemplate, MeritPrerequisite}, book_reference::BookReference};

use super::VariableMeritTemplateBuilderWithDots;

pub struct VariableStackableMeritTemplateBuilder(pub(crate) VariableMeritTemplateBuilderWithDots);

impl VariableStackableMeritTemplateBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.0.book_reference(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.0.prerequisite(prerequisite);
        self
    }

    pub fn dot_option(mut self, dots: u8, description: impl Into<String>) -> Self {
        self.0.dot_option(dots, description);
        self
    }

    pub fn build(self) -> VariableStackableMeritTemplate {
        VariableStackableMeritTemplate {
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