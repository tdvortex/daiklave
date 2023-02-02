use std::collections::HashSet;

use crate::{book_reference::BookReference, merits::merit::{MeritType, MeritPrerequisite, AddStackableMerit, MeritInstanceInner, StackableMeritInstance}};

use super::StackableMeritTemplateName;

pub struct FixedStackableMeritTemplate {
    name: StackableMeritTemplateName,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    description: String,
    prerequisites: HashSet<MeritPrerequisite>,
    dots: u8
}

impl FixedStackableMeritTemplate {
    pub fn instance(self, detail: impl Into<String>) -> AddStackableMerit {
        let inner = MeritInstanceInner {
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            description: self.description,
            prerequisites: self.prerequisites,
            dots: self.dots,
            dot_description: None,
        };
        let instance = StackableMeritInstance(inner);
        
        AddStackableMerit {
            template_name: self.name,
            detail: detail.into(),
            instance,
        }
    }
}