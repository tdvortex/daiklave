use serde::{Serialize, Deserialize};

use crate::{
    merits::merit::{template::StackableMeritTemplateName, AddMerit},
    CharacterMutation,
};

use super::StackableMeritInstance;

/// A mutation to add a stackable merit to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddStackableMerit {
    pub(crate) template_name: StackableMeritTemplateName,
    pub(crate) detail: String,
    pub(crate) instance: StackableMeritInstance,
}

impl From<AddStackableMerit> for CharacterMutation {
    fn from(add_stackable: AddStackableMerit) -> Self {
        AddMerit::from(add_stackable).into()
    }
}
