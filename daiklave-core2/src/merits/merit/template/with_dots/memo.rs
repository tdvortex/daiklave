use crate::{book_reference::BookReference, merits::merit::{MeritType, prerequisite::MeritPrerequisite}};

use super::MeritTemplateWithDots;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MeritTemplateWithDotsMemo {
    name: String,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    shared_description: String,
    dot_description: String,
    prerequisites: Vec<MeritPrerequisite>,
}

impl<'source> MeritTemplateWithDotsMemo {
    pub fn as_ref(&'source self) -> MeritTemplateWithDots<'source> {
        MeritTemplateWithDots {
            name: self.name.as_ref(),
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            shared_description: self.shared_description.as_str(),
            dot_description: self.shared_description.as_str(),
            prerequisites: self.prerequisites.clone(),
        }
    }
}