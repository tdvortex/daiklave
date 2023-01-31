use crate::merits::merit::{StackableMeritTemplateName, StackableMerit, StackableMeritTemplate};

pub struct AddStackableMerit {
    template_name: StackableMeritTemplateName,
    detail: String,
    merit: StackableMerit,
}

impl AddStackableMerit {
    pub fn new(template: StackableMeritTemplate, detail: impl Into<String>, dots: u8) -> Result<Self, MeritError> {
        template.instance(dots, detail)
    }
}