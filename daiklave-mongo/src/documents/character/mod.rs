mod new;
mod versions;
pub use new::NewCharacter;
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

