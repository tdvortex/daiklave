mod create;
mod versions;
pub use create::CreateCharacter;
use serde::{Serialize, Deserialize};
pub use versions::{CharacterCurrent, CharacterV0};

/// A versioned Character document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "character")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum CharacterDocument {
    /// Version 0
    V0(CharacterV0),
}

impl From<CharacterCurrent> for CharacterDocument {
    fn from(value: CharacterCurrent) -> Self {
        Self::V0(value)
    }
}

impl From<CharacterDocument> for CharacterCurrent {
    fn from(value: CharacterDocument) -> Self {
        match value {
            CharacterDocument::V0(value) => value,
        }
    }
}