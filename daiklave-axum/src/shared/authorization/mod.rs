mod get_campaign;
mod get_channel;

pub use get_campaign::GetCampaignAuthorization;
pub use get_channel::GetChannelAuthorization;

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serenity::all::UserId;

/// The authorization of a user with regards to a specific campaign.
#[derive(Debug)]
pub struct Authorization {
    /// The Discord snowflake of the user with authorization
    pub user_id: UserId,
    /// The campaign the authorization applies to.
    pub campaign_id: ObjectId,
    /// Whether the user is the storyteller of the campaign.
    pub is_storyteller: bool,
    /// The active character for this campaign, if any.
    pub active_character: Option<ObjectId>,
}

/// The cache entry by (userId, campaignId).
#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignAuthorization {
    /// Whether the user is the storyteller of the campaign.
    pub is_storyteller: bool,
}

/// The cache entry by (userId, channelId).
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelAuthorization {
    /// The campaign the authorization applies to.
    pub campaign_id: ObjectId,
    /// The active character for this campaign, if any.
    pub active_character: Option<ObjectId>,
    /// The channel of the campaign
    /// Whether the user is the storyteller of the campaign.
    pub is_storyteller: bool,
}