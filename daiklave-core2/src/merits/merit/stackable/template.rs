use crate::merits::merit::template::MeritTemplate;

use super::StackableMeritTemplateId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackableMeritTemplate(
    pub(crate) StackableMeritTemplateId,
    pub(crate) MeritTemplate,
);
