mod create;
mod delete;
mod retrieve;
mod switch;
mod update;
mod versions;
pub use create::CreateCharacter;
pub use delete::DeleteCharacter;
pub use retrieve::{GetCharacter, ListCharacters};
pub use switch::{SwitchCharacter};
pub use update::UpdateCharacter;
pub use versions::{CharacterCurrent, CharacterV0};
use serde::{Serialize, Deserialize};

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