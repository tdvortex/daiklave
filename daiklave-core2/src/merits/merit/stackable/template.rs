use crate::merits::merit::template::MeritTemplate;

use super::StackableMeritTemplateId;

pub struct StackableMeritTemplate(
    pub(crate) StackableMeritTemplateId,
    pub(crate) MeritTemplate,
);
