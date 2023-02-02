use std::collections::HashSet;

use crate::{merits::merit::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::FixedMeritTemplateBuilderWithDescription;

/// A fixed-dot merit builder after the merit type has been defined.
pub struct FixedMeritTemplateBuilderWithMeritType {
    pub(crate) name: String,
    pub(crate) dots: u8,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) book_reference: Option<BookReference>,
}

impl FixedMeritTemplateBuilderWithMeritType {
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

    /// Sets the description for the merit.
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

