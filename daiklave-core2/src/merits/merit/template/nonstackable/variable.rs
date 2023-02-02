use std::collections::{HashMap, HashSet};

use crate::{book_reference::BookReference, merits::merit::{MeritType, MeritPrerequisite, AddNonStackableMerit, MeritError, MeritInstanceInner, NonStackableMeritInstance, template::builder::VariableNonStackableMeritTemplateBuilder}};

use super::NonStackableMeritTemplateName;

pub struct VariableNonStackableMeritTemplate {
    name: NonStackableMeritTemplateName,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    description: String,
    prerequisites: HashSet<MeritPrerequisite>,
    min_dots: (u8, String),
    other_dots: HashMap<u8, String>,
}

impl From<VariableNonStackableMeritTemplateBuilder> for VariableNonStackableMeritTemplate {
    fn from(builder: VariableNonStackableMeritTemplateBuilder) -> Self {
        builder.build()
    }
}


impl VariableNonStackableMeritTemplate {
    pub fn instance(mut self, dots: u8) -> Result<AddNonStackableMerit, MeritError> {
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
        let instance = NonStackableMeritInstance(inner);
        
        Ok(AddNonStackableMerit {
            name: self.name,
            instance,
        })
    }
}
