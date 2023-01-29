use crate::merits::merit::{NonStackableMerit, nonstackable::{NonStackableMeritTemplate, NonStackableMeritName}, MeritError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddNonStackableMerit {
    name: NonStackableMeritName,
    merit: NonStackableMerit,
}

impl AddNonStackableMerit {
    pub fn new(template: NonStackableMeritTemplate, dots: u8) -> Result<Self, MeritError> {
        template.with_dots(dots)
    }
}