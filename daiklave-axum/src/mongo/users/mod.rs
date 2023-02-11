use serde::{Serialize, Deserialize};

use self::versions::{UserV0};
pub use versions::UserCurrent;

mod player_campaign;
mod versions;

/// A versioned User document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "user")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum UserDocument {
    /// Version zero.
    V0(UserV0),   
}

impl From<UserCurrent> for UserDocument {
    fn from(value: UserCurrent) -> Self {
        Self::V0(value)
    }
}

impl From<UserDocument> for UserCurrent {
    fn from(value: UserDocument) -> Self {
        match value {
            UserDocument::V0(value) => value,
        }
    }
}