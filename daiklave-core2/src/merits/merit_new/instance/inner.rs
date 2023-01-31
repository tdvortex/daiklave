use std::collections::HashSet;

use crate::{book_reference::BookReference, merits::merit_new::{merit_type::MeritType, prerequisite::MeritPrerequisite}};

pub(crate) struct MeritInstanceInner {
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    description: String,
    prerequisites: HashSet<MeritPrerequisite>,
    dots: u8,
    dot_description: Option<String>,
}