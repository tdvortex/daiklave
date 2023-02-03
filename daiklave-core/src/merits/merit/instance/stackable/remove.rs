use crate::merits::merit::template::StackableMeritTemplateName;

/// A mutation to remove a stackable merit from the character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveStackableMerit {
    /// The name of the merit to remove, like "Allies".
    pub template_name: StackableMeritTemplateName,
    /// The detail of the merit that needs removing, like a specific ally.
    pub detail: String,
}
