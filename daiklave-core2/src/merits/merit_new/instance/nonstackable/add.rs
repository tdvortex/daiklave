use crate::merits::merit_new::template::NonStackableMeritTemplateName;

use super::NonStackableMeritInstance;

pub struct AddNonStackableMerit {
    name: NonStackableMeritTemplateName,
    instance: NonStackableMeritInstance,
}