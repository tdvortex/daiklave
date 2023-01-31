use crate::merits::merit::{template::MeritTemplate, add::AddStackableMerit, MeritError};

use super::{StackableMeritTemplateName, StackableMerit};

/// A merit template which can be purchased more than once. Each time the merit
/// is added, the a detail must be provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackableMeritTemplate(pub(crate) StackableMeritTemplateName, pub(crate) MeritTemplate);

impl StackableMeritTemplate {
    pub fn instance(self, dots: u8, detail: impl Into<String>) -> Result<AddStackableMerit, MeritError> {
        Ok(AddStackableMerit {
            template_name: self.0,
            detail: detail.into(),
            merit: StackableMerit
        })
    }
}
