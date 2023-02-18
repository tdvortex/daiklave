mod v0;
use serde::{Deserialize, Serialize};
pub use v0::UserV0;

/// The current version of the User document.
pub type UserCurrent = UserV0;

/// A version tag for the user struct to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserVersion {
    /// Version zero
    V0,
}
