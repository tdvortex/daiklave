use crate::{
    merits::merit::{
        template::{NonStackableMeritName, NonStackableMeritTemplateName},
        RemoveMerit,
    },
    CharacterMutation,
};

/// A mutation to remove a nonstackable merit from a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveNonStackableMerit {
    name: NonStackableMeritTemplateName,
}

impl RemoveNonStackableMerit {
    /// Removes a merit with this name.
    pub fn name(name: impl Into<NonStackableMeritName>) -> Self {
        Self { name: name.into() }
    }
}

impl From<RemoveNonStackableMerit> for CharacterMutation {
    fn from(remove_non_stackable: RemoveNonStackableMerit) -> Self {
        RemoveMerit::NonStackable(remove_non_stackable).into()
    }
}
