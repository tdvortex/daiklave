use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    merits::merit::{MeritPrerequisite, MeritType},
};

use super::VariableMeritTemplateBuilderWithDescription;

/// A variable-dot merit template builder after the merit type is specified.
pub struct VariableMeritTemplateBuilderWithMeritType {
    pub(crate) name: String,
    pub(crate) merit_type: MeritType,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilderWithMeritType {
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

    /// Adds a description to the merit. This is the blanket description and
    /// applies for all dot levels.
    pub fn description(
        self,
        description: impl Into<String>,
    ) -> VariableMeritTemplateBuilderWithDescription {
        VariableMeritTemplateBuilderWithDescription {
            name: self.name,
            merit_type: self.merit_type,
            description: description.into(),
            book_reference: self.book_reference,
            prerequisites: self.prerequisites,
        }
    }
}
