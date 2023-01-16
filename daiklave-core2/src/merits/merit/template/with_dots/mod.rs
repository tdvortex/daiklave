mod memo;
pub(crate) use memo::MeritTemplateWithDotsMemo;

use crate::{
    book_reference::BookReference,
    merits::merit::{prerequisite::MeritPrerequisite, MeritType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MeritTemplateWithDots<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    shared_description: &'source str,
    dot_description: Option<&'source str>,
    prerequisites: &'source [MeritPrerequisite],
}

impl<'source> MeritTemplateWithDots<'source> {
    pub fn as_memo(&self) -> MeritTemplateWithDotsMemo {
        MeritTemplateWithDotsMemo {
            name: self.name.to_owned(),
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            shared_description: self.shared_description.to_owned(),
            dot_description: self.dot_description.map(|s| s.to_owned()),
            prerequisites: self.prerequisites.iter().copied().collect(),
        }
    }

    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn merit_type(&self) -> MeritType {
        self.merit_type
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        (self.shared_description, self.dot_description)
    }
}
