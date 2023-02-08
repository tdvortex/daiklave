use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serenity::all::UserId;

use crate::{PlayerCampaign, user::UserDocument};

/// Version zero of the User document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "UserDocument")]
#[serde(from = "UserDocument")]
pub struct UserV0 {
    /// The MongoDB ID field for this user.
    pub _id: ObjectId,
    /// The Discord snowflake for this user.
    pub discord_id: UserId,
    /// The campaigns that this player is a part of.
    #[serde(default)]
    pub campaigns: Vec<PlayerCampaign>,
}