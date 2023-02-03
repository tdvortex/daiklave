use crate::{
    merits::merit::{template::NonStackableMeritTemplateName, AddMerit},
    CharacterMutation,
};

use super::NonStackableMeritInstance;

/// A mutation to add a non-stackable merit to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddNonStackableMerit {
    pub(crate) name: NonStackableMeritTemplateName,
    pub(crate) instance: NonStackableMeritInstance,
}

impl From<AddNonStackableMerit> for CharacterMutation {
    fn from(add_ns_merit: AddNonStackableMerit) -> Self {
        AddMerit::from(add_ns_merit).into()
    }
}
