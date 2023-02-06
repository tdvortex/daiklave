use std::collections::HashMap;

use bson::oid::ObjectId;
use serenity::all::UserId;

use crate::PlayerCampaign;

/// Version zero of the User document.
pub struct UserV0 {
    /// The MongoDB ID field for this user.
    pub _id: ObjectId,
    /// The Discord snowflake for this user.
    pub discord_id: UserId,
    /// The campaigns that this player is a part of.
    pub campaigns: HashMap<ObjectId, PlayerCampaign>,
}