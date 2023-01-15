mod dot_options;
pub(crate) use dot_options::MeritTemplateDotOptions;

mod id;
pub use id::MeritTemplateId;

mod with_dots;
pub(crate) use with_dots::{MeritTemplateWithDots, MeritTemplateWithDotsMemo};

use crate::book_reference::BookReference;

use super::{MeritType, MeritError, prerequisite::MeritPrerequisite};

pub struct MeritTemplate {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) shared_description: String,
    pub(crate) dot_options: MeritTemplateDotOptions,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
}

impl MeritTemplate {
    pub(crate) fn set_dots(self, dots: u8) -> Result<MeritTemplateWithDotsMemo, MeritError> {
        match self.dot_options {
            MeritTemplateDotOptions::Fixed(fixed) => {
                if dots != fixed {
                    Err(MeritError::InvalidDotRating)
                } else {
                    Ok(
                        MeritTemplateWithDotsMemo { 
                            name: self.name,
                            book_reference: self.book_reference,
                            merit_type: self.merit_type,
                            shared_description: self.shared_description,
                            dot_description: None,
                            prerequisites: self.prerequisites,
                        }
                    )
                }
            }
            MeritTemplateDotOptions::Variable(mut options) => {
                if !(0..=5).contains(&dots) {
                    Err(MeritError::InvalidDotRating)
                } else if options[dots as usize].is_none() {
                    Err(MeritError::InvalidDotRating)
                } else {
                    Ok(
                        MeritTemplateWithDotsMemo { 
                            name: self.name,
                            book_reference: self.book_reference,
                            merit_type: self.merit_type,
                            shared_description: self.shared_description,
                            dot_description: options[dots as usize].take(),
                            prerequisites: self.prerequisites,
                        }
                    )
                }
            }
        }
    }
}