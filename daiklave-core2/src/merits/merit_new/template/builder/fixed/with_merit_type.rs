use std::collections::HashSet;

use crate::{merits::merit_new::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::FixedMeritTemplateBuilderWithDescription;

pub struct FixedMeritTemplateBuilderWithMeritType {
    pub(crate) name: String,
    pub(crate) dots: u8,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) book_reference: Option<BookReference>,
}

impl FixedMeritTemplateBuilderWithMeritType {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn description(self, description: impl Into<String>) -> FixedMeritTemplateBuilderWithDescription {
        FixedMeritTemplateBuilderWithDescription {
            name: self.name,
            dots: self.dots,
            merit_type: self.merit_type,
            description: description.into(),
            prerequisites: self.prerequisites,
            book_reference: self.book_reference,
        }
    }
}

