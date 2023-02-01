use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, merits::merit_new::{merit_type::MeritType, prerequisite::MeritPrerequisite}};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MeritInstanceInner {
    pub book_reference: Option<BookReference>,
    pub merit_type: MeritType,
    pub description: String,
    pub prerequisites: HashSet<MeritPrerequisite>,
    pub dots: u8,
    pub dot_description: Option<String>,
}