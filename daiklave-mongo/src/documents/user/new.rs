use std::collections::HashMap;

use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serenity::all::UserId;

use crate::PlayerCampaign;

/// A document to insert a new User.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "user")]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    /// The version of the User document to be inserted.
    pub version: String,
    /// The Discord snowflake for this user.
    pub discord_id: UserId,
    /// The campaigns that this player is a part of.
    pub campaigns: HashMap<ObjectId, PlayerCampaign>,
}