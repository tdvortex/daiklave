mod get_campaign;
mod get_channel;

pub use get_campaign::GetCampaignAuthorization;
pub use get_channel::GetChannelAuthorization;

use mongodb::bson::oid::ObjectId;
use serenity::all::UserId;

/// The authorization of a user with regards to a specific campaign.
pub struct Authorization {
    /// The Discord snowflake of the user with authorization
    pub user_id: UserId,
    /// The campaign the authorization applies to.
    pub campaign_id: ObjectId,
    /// Whether the user is the storyteller of the campaign.
    pub is_storyteller: bool,
}