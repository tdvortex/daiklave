use bson::oid::ObjectId;
use mongodb::ClientSession;
use serenity::all::UserId;

use crate::error::DocumentError;

/// Removes a player from a campaign. The storyteller cannot be removed.
pub struct RemoveCampaignPlayer {
    /// The Id of the campaign.
    pub campaign_id: ObjectId,
    /// The Id of the user to add.
    pub user_id: UserId,
}

impl RemoveCampaignPlayer {
    /// Removes the player from the campaign. Requires a session to update 
    /// campaigns, players, and characters atomically.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut ClientSession) -> Result<(), DocumentError> {
        todo!()
    }
}