use crate::{merits::merit_new::{template::nonstackable::FixedNonStackableMeritTemplate, MeritPrerequisite}, book_reference::BookReference};

use super::FixedMeritTemplateBuilderWithDescription;

pub struct FixedNonStackableMeritTemplateBuilder(pub(crate) FixedMeritTemplateBuilderWithDescription);

impl FixedNonStackableMeritTemplateBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self = Self(self.0.book_reference(book_reference));
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self = Self(self.0.prerequisite(prerequisite));
        self
    }

    pub fn build(self) -> FixedNonStackableMeritTemplate {
        FixedNonStackableMeritTemplate {
            name: self.0.name.into(),
            book_reference: self.0.book_reference,
            merit_type: self.0.merit_type,
            description: self.0.description,
            prerequisites: self.0.prerequisites,
            dots: self.0.dots,
        }
    }
}