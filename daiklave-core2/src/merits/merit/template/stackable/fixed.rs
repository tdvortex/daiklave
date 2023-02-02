use std::collections::HashSet;

use crate::{book_reference::BookReference, merits::merit::{MeritType, MeritPrerequisite, AddStackableMerit, MeritInstanceInner, StackableMeritInstance}};

use super::StackableMeritTemplateName;

pub struct FixedStackableMeritTemplate {
    pub(crate) name: StackableMeritTemplateName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
    pub(crate) dots: u8
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