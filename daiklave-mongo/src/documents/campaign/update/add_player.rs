use bson::{oid::ObjectId, doc};
use mongodb::ClientSession;
use serenity::all::UserId;

use crate::{error::DocumentError, user::{UserCurrent, CreateUser, UserVersion}, campaign::CampaignCurrent, PlayerCampaign};

/// Add a player to a campaign, as a non-storyteller. The user document will 
/// be created if it does not already exist.
pub struct AddCampaignPlayer {
    /// The Id of the campaign to add the player to.
    pub campaign_id: ObjectId,
    /// The Id of the user to add.
    pub user_id: UserId,
}

impl AddCampaignPlayer {
    /// Adds a player to the campaign. Requires a session to update campaign 
    /// and player atomically.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut ClientSession) -> Result<(), DocumentError> {
        session.start_transaction(None).await?;

        let player_bson = bson::to_bson(&self.user_id).or(Err(DocumentError::SerializationError))?;

        // Get the campaign document
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self.campaign_id
        };
        let mut campaign = campaigns.find_one_with_session(filter, None, session).await?.ok_or(DocumentError::NotFound)?;

        // Add the player to the campaign
        if !campaign.players.contains(&self.user_id) {
            campaign.players.insert(self.user_id);
            let query = doc! {
                "_id": self.campaign_id
            };
            campaigns.replace_one_with_session(query, &campaign, None, session).await?;
        } else {
            // Nothing to do, early exit
            session.commit_transaction().await?;
            return Ok(());
        }

        // Get the player document, if it exists
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": &player_bson
        };
        let maybe_player = users.find_one_with_session(filter, None, session).await?;

        if let Some(mut player) = maybe_player {
            // Update and replace the existing player document
            if player.campaigns.iter().find(|player_campaign| player_campaign.campaign_id == self.campaign_id).is_none() {
                player.campaigns.push(PlayerCampaign {
                    campaign_id: self.campaign_id,
                    name: campaign.name,
                    is_storyteller: false,
                    characters: Default::default(),
                    dice_channel: campaign.dice_channel,
                    channels: campaign.channels,
                });
            }
            let query = doc! {
                "discordId": &player_bson
            };
            users.replace_one_with_session(query, player, None, session).await?;
        } else {
            // Create a new user with this campaign.
            let new_user = CreateUser {
                version: UserVersion::V0,
                discord_id: self.user_id,
                campaigns: vec![PlayerCampaign {
                    campaign_id: self.campaign_id,
                    name: campaign.name,
                    is_storyteller: false,
                    characters: Default::default(),
                    dice_channel: campaign.dice_channel,
                    channels: campaign.channels,
                }]
            };
            let users = database.collection::<CreateUser>("users");
            users.insert_one_with_session(new_user, None, session).await?;
        }

        // Done with database
        session.commit_transaction().await?;
        Ok(())
    }
}