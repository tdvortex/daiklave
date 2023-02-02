mod nonstackable;
mod stackable;
mod with_description;
mod with_merit_type;
pub use nonstackable::FixedNonStackableMeritTemplateBuilder;
pub use stackable::FixedStackableMeritTemplateBuilder;
pub use with_description::FixedMeritTemplateBuilderWithDescription;
pub use with_merit_type::FixedMeritTemplateBuilderWithMeritType;

use std::collections::HashSet;

use crate::{merits::{merit::{MeritPrerequisite, MeritType}}, book_reference::BookReference};

pub struct FixedMeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) dots: u8,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) book_reference: Option<BookReference>,
}

impl FixedMeritTemplateBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn merit_type(self, merit_type: MeritType) -> FixedMeritTemplateBuilderWithMeritType {
        FixedMeritTemplateBuilderWithMeritType {
            name: self.name,
            dots: self.dots,
            merit_type,
            prerequisites: self.prerequisites,
            book_reference: self.book_reference,
        }
    }
}

