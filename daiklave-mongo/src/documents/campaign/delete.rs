use bson::{doc, oid::ObjectId};
use serenity::all::{ChannelId, UserId};

use crate::{
    channel::ChannelCurrent, character::CharacterCurrent, error::DocumentError, user::UserCurrent,
};

use super::CampaignCurrent;

/// An instruction to delete a campaign and all of its content. Be very sure
/// that the user is authorized to do this!
pub struct DeleteCampaign {
    /// The Id of the campaign to be deleted.
    pub campaign_id: ObjectId,
    /// The Id of the user requesting the delete--must match the storyteller
    /// field of the campaign document.
    pub storyteller: UserId,
}

impl DeleteCampaign {
    /// Deletes the campaign. Requires a session for multi-collection 
    /// operations. Returns a vector of pairs of (channel_id, player_id) 
    /// permissions which must be invalidated in the cache.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<Vec<(ChannelId, UserId)>, DocumentError> {
        session.start_transaction(None).await?;

        // Get the campaign
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self.campaign_id,
        };
        let campaign = campaigns
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or(DocumentError::NotFound)?;
        if campaign.storyteller != self.storyteller {
            return Err(DocumentError::Unauthorized);
        }

        let dropped_channels = campaign.channels;
        let players = campaign.players;

        // Delete all of the documents for channels used by this campaign
        let channels = database.collection::<ChannelCurrent>("channels");
        let query = doc! {
            "campaignId": self.campaign_id,
        };
        channels
            .delete_many_with_session(query, None, session)
            .await?;

        // Delete the campaign subdocument for all players in this campaign
        let users = database.collection::<UserCurrent>("users");
        let query = doc! {};
        let update = doc! {
            "$pull": {
                "campaigns": {
                    "campaignId": self.campaign_id,
                }
            }
        };
        users
            .update_many_with_session(query, update, None, session)
            .await?;

        // Delete all the content for this campaign
        // Characters
        let characters = database.collection::<CharacterCurrent>("characters");
        let query = doc! {
            "campaignId": self.campaign_id,
        };
        characters
            .delete_many_with_session(query, None, session)
            .await?;

        // Done with the database
        session.commit_transaction().await?;

        // Return all of the channel-player pairs to invalidate in the cache
        Ok(dropped_channels
            .into_iter()
            .flat_map(|channel_id| players.iter().map(move |&user_id| (channel_id, user_id)))
            .collect::<Vec<(ChannelId, UserId)>>())
    }
}
