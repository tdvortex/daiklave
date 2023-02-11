use bson::{oid::ObjectId, doc};
use serenity::all::UserId;

use crate::{error::DocumentError, campaign::CampaignCurrent, user::{UserCurrent, CreateUser, UserVersion}, PlayerCampaign};

/// Reassigns the storyteller for this campaign. The storyteller document will 
/// be created if it does not exist, and will be added to the campaign if not
/// already a part of it. The previous storyteller (the one invokind this 
/// command) will be left as a player in the campaign.
pub struct SetStoryteller {
    /// The Id of the campaign.
    pub campaign_id: ObjectId,
    /// The Id of the new storyteller.
    pub user_id: UserId,
}

impl SetStoryteller {
    /// Reassigns the storyteller. Requires a session to update campaign and 
    /// players atomically.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<(), DocumentError> {
        session.start_transaction(None).await?;

        let player_bson = bson::to_bson(&self.user_id).or(Err(DocumentError::SerializationError))?;

        // Get the campaign document
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self.campaign_id
        };
        let mut campaign = campaigns.find_one_with_session(filter, None, session).await?.ok_or(DocumentError::NotFound)?;

        if campaign.storyteller == self.user_id {
            // Nothing to do if player is already storyteller
            session.commit_transaction().await?;
            return Ok(());
        }

        // Update the campaign to have this player as both a player and storyteller
        campaign.players.insert(self.user_id);
        campaign.storyteller = self.user_id;
        let query = doc! {
            "_id": self.campaign_id
        };
        campaigns.replace_one_with_session(query, &campaign, None, session).await?;

        // Get the player document if it exists
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": &player_bson
        };
        let maybe_user = users.find_one_with_session(filter, None, session).await?;

        if let Some(mut user) = maybe_user {
            if let Some(existing_campaign) = user.campaigns.iter_mut().find(|player_campaign| player_campaign.campaign_id == self.campaign_id) {
                // Update the existing campaign document to have is_storyteller = true
                existing_campaign.is_storyteller = true;
            } else {
                // Add the campaign if it doesn't already exist
                let added_campaign = PlayerCampaign {
                    campaign_id: self.campaign_id,
                    name: campaign.name.clone(),
                    is_storyteller: true,
                    characters: Default::default(),
                    dice_channel: campaign.dice_channel,
                    channels: campaign.channels,
                };
                user.campaigns.push(added_campaign);
            }

            // Replace the existing user document
            let query = doc! {
                "discordId": &player_bson
            };
            users.replace_one_with_session(query, user, None, session).await?;
        } else {
            // Add a new user document
            let user = CreateUser {
                version: UserVersion::V0,
                discord_id: self.user_id,
                campaigns: vec![PlayerCampaign {
                    campaign_id: self.campaign_id,
                    name: campaign.name.clone(),
                    is_storyteller: true,
                    characters: Default::default(),
                    dice_channel: campaign.dice_channel,
                    channels: campaign.channels,
                }]
            };
            let users = database.collection::<CreateUser>("users");
            users.insert_one_with_session(user, None, session).await?;
        }

        // Done with database
        session.commit_transaction().await?;
        Ok(())
    }
} 