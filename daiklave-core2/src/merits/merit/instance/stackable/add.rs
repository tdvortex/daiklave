use crate::merits::merit::StackableMeritTemplateName;

use super::StackableMeritInstance;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddStackableMerit {
    pub(crate) template_name: StackableMeritTemplateName,
    pub(crate) detail: String,
    pub(crate) instance: StackableMeritInstance,
}
