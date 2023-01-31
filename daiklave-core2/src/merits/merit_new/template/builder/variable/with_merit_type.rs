use std::collections::HashSet;

use crate::{merits::merit_new::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::VariableMeritTemplateBuilderWithDescription;

pub struct VariableMeritTemplateBuilderWithMeritType {
    name: String,
    merit_type: MeritType,
    book_reference: Option<BookReference>,
    prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilderWithMeritType {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn description(self, description: impl Into<String>) -> VariableMeritTemplateBuilderWithDescription {
        VariableMeritTemplateBuilderWithDescription {
            name: self.name,
            merit_type: self.merit_type,
            description: description.into(),
            book_reference: self.book_reference,
            prerequisites: self.prerequisites,
        }
    }
}