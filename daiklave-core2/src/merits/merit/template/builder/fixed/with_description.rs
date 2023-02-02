use std::collections::HashSet;

use crate::{merits::merit::{MeritType, MeritPrerequisite}, book_reference::BookReference};

use super::{FixedStackableMeritTemplateBuilder, FixedNonStackableMeritTemplateBuilder};

pub struct FixedMeritTemplateBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) dots: u8,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) book_reference: Option<BookReference>,
}

impl FixedMeritTemplateBuilderWithDescription {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn nonstackable(self) -> FixedNonStackableMeritTemplateBuilder {
        FixedNonStackableMeritTemplateBuilder(self)
    }

    pub fn stackable(self) -> FixedStackableMeritTemplateBuilder {
        FixedStackableMeritTemplateBuilder(self)
    }
}
