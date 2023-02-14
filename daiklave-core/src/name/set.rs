use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

/// A mutation to set the character's name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetName(pub String);

impl SetName {
    /// Creates a new mutation to set the character's name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<SetName> for CharacterMutation {
    fn from(set_name: SetName) -> Self {
        CharacterMutation::SetName(set_name)
    }
}
