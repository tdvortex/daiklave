mod nonstackable;
mod stackable;
mod with_description;
mod with_dots;
mod with_merit_type;
pub use nonstackable::VariableNonStackableMeritTemplateBuilder;
pub use stackable::VariableStackableMeritTemplateBuilder;
pub use with_description::VariableMeritTemplateBuilderWithDescription;
pub use with_dots::VariableMeritTemplateBuilderWithDots;
pub use with_merit_type::VariableMeritTemplateBuilderWithMeritType;

use std::collections::HashSet;

use crate::{merits::merit_new::{MeritPrerequisite, MeritType}, book_reference::BookReference};

pub struct VariableMeritTemplateBuilder {
    name: String,
    book_reference: Option<BookReference>,
    prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilder {
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            prerequisites: Default::default(),
            book_reference: Default::default(),
        }
    }

    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn with_prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn merit_type(self, merit_type: MeritType) -> VariableMeritTemplateBuilderWithMeritType {
        VariableMeritTemplateBuilderWithMeritType {
            name: self.name,
            merit_type,
            book_reference: self.book_reference,
            prerequisites: self.prerequisites,
        }
    }
}