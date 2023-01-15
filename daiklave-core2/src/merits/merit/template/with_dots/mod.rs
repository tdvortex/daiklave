mod memo;
pub(crate) use memo::MeritTemplateWithDotsMemo;

use crate::{book_reference::BookReference, merits::merit::{MeritType, prerequisite::MeritPrerequisite}};

pub(crate) struct MeritTemplateWithDots<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    shared_description: &'source str,
    dot_description: &'source str,
    prerequisites: Vec<MeritPrerequisite>,
}

impl<'source> MeritTemplateWithDots<'source> {
    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn merit_type(&self) -> MeritType {
        self.merit_type
    }

    pub fn description(&self) -> (&'source str, &'source str) {
        (self.shared_description, self.dot_description)
    }

    pub fn prerequisites(&self) -> impl Iterator<Item = MeritPrerequisite> + '_ {
        self.prerequisites.iter().copied()
    }
}