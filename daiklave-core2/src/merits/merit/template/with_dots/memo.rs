use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    merits::merit::{prerequisite::MeritPrerequisite, MeritType},
};

use super::MeritTemplateWithDots;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MeritTemplateWithDotsMemo {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub merit_type: MeritType,
    pub shared_description: String,
    pub dot_description: Option<String>,
    pub prerequisites: Vec<MeritPrerequisite>,
}

impl<'source> MeritTemplateWithDotsMemo {
    pub fn as_ref(&'source self) -> MeritTemplateWithDots<'source> {
        MeritTemplateWithDots {
            name: self.name.as_ref(),
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            shared_description: self.shared_description.as_str(),
            dot_description: self.dot_description.as_deref(),
            prerequisites: &self.prerequisites,
        }
    }
}
