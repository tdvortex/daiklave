use crate::merits::merit::{template::MeritTemplate, add::AddNonStackableMerit, MeritError};

use super::{NonStackableMerit, NonStackableMeritName};

pub struct NonStackableMeritTemplate(pub(crate) NonStackableMeritName, pub(crate) MeritTemplate);

impl NonStackableMeritTemplate {
    pub fn with_dots(self, dots: u8) -> Result<AddNonStackableMerit, MeritError> {
        NonStackableMerit::new(self, dots)
    }
}