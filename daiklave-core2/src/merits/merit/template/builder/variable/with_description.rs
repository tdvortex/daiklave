use std::collections::{HashSet, HashMap};

use crate::{merits::merit::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::VariableMeritTemplateBuilderWithDots;

pub struct VariableMeritTemplateBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilderWithDescription {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn dot_option(self, dots: u8, description: impl Into<String>) -> VariableMeritTemplateBuilderWithDots {
        VariableMeritTemplateBuilderWithDots {
            name: self.name,
            merit_type: self.merit_type,
            description: self.description,
            min_dots: (dots, description.into()),
            other_dots: HashMap::new(),
            book_reference: self.book_reference,
            prerequisites: self.prerequisites,
        }
    }
}