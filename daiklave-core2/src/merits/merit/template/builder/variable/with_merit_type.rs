use std::collections::HashSet;

use crate::{merits::merit::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::VariableMeritTemplateBuilderWithDescription;

pub struct VariableMeritTemplateBuilderWithMeritType {
    pub(crate) name: String,
    pub(crate) merit_type: MeritType,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
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