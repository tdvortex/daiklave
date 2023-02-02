use crate::merits::merit::template::NonStackableMeritTemplateName;

use super::NonStackableMeritInstance;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddNonStackableMerit {
    pub(crate) name: NonStackableMeritTemplateName,
    pub(crate) instance: NonStackableMeritInstance,
}