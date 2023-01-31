use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    merits::merit_new::{
        instance::{MeritInstanceInner, NonStackableMeritInstance},
        merit_type::MeritType,
        prerequisite::MeritPrerequisite,
        AddNonStackableMerit,
    },
};

use super::name::NonStackableMeritTemplateName;

pub struct FixedNonStackableMeritTemplate {
    name: NonStackableMeritTemplateName,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    description: String,
    prerequisites: HashSet<MeritPrerequisite>,
    dots: u8,
}

impl FixedNonStackableMeritTemplate {
    pub fn instance(self) -> AddNonStackableMerit {
        let inner = MeritInstanceInner {
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            description: self.description,
            prerequisites: self.prerequisites,
            dots: self.dots,
            dot_description: None,
        };
        let instance = NonStackableMeritInstance(inner);

        AddNonStackableMerit {
            name: self.name,
            instance,
        }
    }
}
