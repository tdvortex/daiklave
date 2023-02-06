mod new;
mod versions;
pub use new::NewUser;
use serde::{Serialize, Deserialize};
pub use versions::{UserCurrent, UserV0};

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