use std::collections::{HashSet, HashMap};

use crate::{merits::merit::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::VariableMeritTemplateBuilderWithDots;

/// A variable-dot merit template builder after the description has been 
/// supplied.
pub struct VariableMeritTemplateBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilderWithDescription {
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

    /// Adds a dot level at which the merit can be purchased. These need not be
    /// consecutive.
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