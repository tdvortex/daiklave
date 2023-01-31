use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    merits::merit::{prerequisite::MeritPrerequisite, MeritType},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MeritTemplateWithDotsMemo {
    pub book_reference: Option<BookReference>,
    pub merit_type: MeritType,
    pub shared_description: String,
    pub prerequisites: Vec<MeritPrerequisite>,
    pub dots: u8,
    pub dot_description: Option<String>,
}
