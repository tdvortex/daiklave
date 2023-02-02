use std::collections::HashSet;

use crate::{merits::merit::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::{FixedStackableMeritTemplateBuilder, FixedNonStackableMeritTemplateBuilder};

/// A fixed-dot merit builder after the description has been supplied.
pub struct FixedMeritTemplateBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) dots: u8,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) book_reference: Option<BookReference>,
}

impl FixedMeritTemplateBuilderWithDescription {
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

    /// Indicates that this merit can only be purchased once per character.
    pub fn nonstackable(self) -> FixedNonStackableMeritTemplateBuilder {
        FixedNonStackableMeritTemplateBuilder(self)
    }

    /// Indicates that this merit may be purchased multiple times per 
    /// character.
    pub fn stackable(self) -> FixedStackableMeritTemplateBuilder {
        FixedStackableMeritTemplateBuilder(self)
    }
}
