use std::collections::HashMap;

use bson::oid::ObjectId;
use serenity::all::UserId;

use crate::PlayerCampaign;

/// A document to insert a new User.
pub struct NewUser {
    /// The version of the User document to be inserted.
    pub version: String,
    /// The Discord snowflake for this user.
    pub discord_id: UserId,
    /// The campaigns that this player is a part of.
    pub campaigns: HashMap<ObjectId, PlayerCampaign>,
}