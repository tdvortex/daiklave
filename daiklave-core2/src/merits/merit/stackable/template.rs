use crate::merits::merit::template::MeritTemplate;

/// A merit template which can be purchased more than once. Each time the merit
/// is added, the a detail must be provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackableMeritTemplate(pub(crate) MeritTemplate);

impl StackableMeritTemplate {
    /// Iterates over the dot options available to instantiate this merit.
    pub fn dot_options(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.dot_options.iter()
    }
}
