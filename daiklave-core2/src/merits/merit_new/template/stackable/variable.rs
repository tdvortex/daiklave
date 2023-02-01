use std::collections::{HashMap, HashSet};

use crate::{book_reference::BookReference, merits::merit_new::{MeritType, MeritPrerequisite, MeritError, MeritInstanceInner, AddStackableMerit, StackableMeritInstance}};

use super::StackableMeritTemplateName;

pub struct VariableStackableMeritTemplate {
    name: StackableMeritTemplateName,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    description: String,
    prerequisites: HashSet<MeritPrerequisite>,
    min_dots: (u8, String),
    other_dots: HashMap<u8, String>,
}

impl VariableStackableMeritTemplate {
    pub fn instance(mut self, dots: u8, detail: impl Into<String>) -> Result<AddStackableMerit, MeritError> {
        let dot_description = if self.min_dots.0 == dots {
            self.min_dots.1
        } else {
            self.other_dots.remove(&dots).ok_or(MeritError::InvalidDotRating)?
        };
        let inner = MeritInstanceInner {
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            description: self.description,
            prerequisites: self.prerequisites,
            dots,
            dot_description: Some(dot_description),
        };
        let instance = StackableMeritInstance(inner);
        
        Ok(AddStackableMerit {
            template_name: self.name,
            detail: detail.into(),
            instance,
        })
    }
}