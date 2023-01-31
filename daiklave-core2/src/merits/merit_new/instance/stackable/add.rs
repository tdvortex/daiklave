use crate::merits::merit_new::StackableMeritTemplateName;

use super::StackableMeritInstance;

pub struct AddStackableMerit {
    template_name: StackableMeritTemplateName,
    detail: String,
    instance: StackableMeritInstance,
}
