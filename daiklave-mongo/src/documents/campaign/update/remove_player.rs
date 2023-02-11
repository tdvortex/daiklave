use bson::{oid::ObjectId, doc};
use mongodb::ClientSession;
use serenity::all::UserId;

use crate::{error::DocumentError, campaign::CampaignCurrent, user::UserCurrent, character::CharacterCurrent};

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
        session.start_transaction(None).await?;

        let player_bson = bson::to_bson(&self.user_id).or(Err(DocumentError::SerializationError))?;

        // Update the campaign document to no longer have this player
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let query = doc! {
            "_id": self.campaign_id
        };
        let update = doc! {
            "$pull": {
                "players": &player_bson
            }
        };
        campaigns.update_one_with_session(query, update, None, session).await?;

        // Remove the campaign from the player
        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "discordId": &player_bson
        };
        let update = doc! {
            "$pull": {
                "campaigns": {
                    "campaignId": self.campaign_id
                }
            }
        };
        users.update_one_with_session(query, update, None, session).await?;

        // Remove all characters this player has with this campaign
        let characters = database.collection::<CharacterCurrent>("characters");
        let query = doc! {
            "player": &player_bson,
            "campaignId": self.campaign_id
        };
        characters.delete_many_with_session(query, None, session).await?;

        // Done with database
        session.commit_transaction().await?;
        Ok(())
    }
}