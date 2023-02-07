mod v0;
use serde::{Serialize, Deserialize};
pub use v0::CharacterV0;

/// The current version of the Character document.
pub type CharacterCurrent = CharacterV0;

/// A version tag for the Character struct to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CharacterVersion {
    /// Version zero
    V0,
}