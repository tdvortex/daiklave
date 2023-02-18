use serde::{Serialize, Deserialize};
use serenity::all::UserId;

use self::versions::{UserV0};
pub use versions::{UserCurrent, UserVersion};

mod player_campaign;
pub use player_campaign::{PlayerCampaign, CharacterStub};
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

/// A new user document, specifying the version but not the _id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsertUser {
    /// The version of the user document.
    pub version: UserVersion,
    /// The Discord snowflake for this user.
    pub discord_id: UserId,
    /// The campaigns that this player is a part of.
    #[serde(default)]
    pub campaigns: Vec<PlayerCampaign>,
}