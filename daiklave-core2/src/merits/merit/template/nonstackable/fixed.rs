use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    merits::merit::{
        instance::{MeritInstanceInner, NonStackableMeritInstance},
        merit_type::MeritType,
        prerequisite::MeritPrerequisite,
        AddNonStackableMerit,
    },
};

use super::name::NonStackableMeritTemplateName;

pub struct FixedNonStackableMeritTemplate {
    pub(crate) name: NonStackableMeritTemplateName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) dots: u8,
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
