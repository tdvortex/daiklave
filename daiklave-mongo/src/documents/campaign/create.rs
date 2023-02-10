use std::collections::HashSet;

use bson::{doc, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, UserId};

use crate::{
    channel::{CreateChannel, ChannelVersion, ChannelCurrent},
    error::DocumentError,
    user::{CreateUser, UserCurrent, UserVersion},
    PlayerCampaign,
};

use super::versions::CampaignVersion;

/// A Campaign document to be inserted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "campaign")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaign {
    /// The version of the Campaign document to be inserted.
    pub version: CampaignVersion,
    /// The human-readable name of the campaign.
    pub name: String,
    /// The Discord Snowflake representing the storyteller for the campaign.
    pub storyteller: UserId,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice
    /// channel)
    pub channels: HashSet<ChannelId>,
}

impl CreateCampaign {
    /// Create a new campaign. Requires ClientSession, as this involves
    /// modifying all of the Users in the campaign simultaneously.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<ObjectId, crate::error::DocumentError> {
        // Start a session to ensure all operations complete atomically
        session.start_transaction(None).await?;

        // Check if any of the channels here is already in use.
        let channel_ids: Vec<Bson> = self
            .channels
            .iter()
            .map(|channel_id| bson::to_bson(channel_id).or(Err(DocumentError::SerializationError)))
            .fold(
                Ok(Vec::new()),
                |acc: Result<Vec<Bson>, DocumentError>, res_bson| match (acc, res_bson) {
                    (Ok(mut v), Ok(b)) => {
                        v.push(b);
                        Ok(v)
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                },
            )?;
        let channels = database.collection::<ChannelCurrent>("channels");
        let filter = doc! {
            "channelId": {
                "$in": channel_ids
            }
        };
        let maybe_channel = channels
            .find_one_with_session(filter, None, session)
            .await?;
        if maybe_channel.is_some() {
            return Err(DocumentError::DuplicateChannelCampaign);
        }

        // Get a handle to the "campaigns" collection, but serializing document using CreateCampaign
        let campaigns = database.collection::<CreateCampaign>("campaigns");

        // Create a new document for this campaign
        let mongodb::results::InsertOneResult { inserted_id, .. } = campaigns
            .insert_one_with_session(self, None, session)
            .await?;
        let new_campaign_id = inserted_id
            .as_object_id()
            .ok_or(DocumentError::DeserializationError)?;

        // Get the storyteller's document, if it exists
        let users = database.collection::<UserCurrent>("users");
        let storyteller_bson = bson::to_bson(&self.storyteller).or(Err(DocumentError::SerializationError))?;
        let filter = doc! {
            "discordId": &storyteller_bson
        };
        let maybe_player = users.find_one_with_session(filter, None, session).await?;

        if let Some(mut player) = maybe_player {
            // Update and replace the existing player document
            if player.campaigns.iter().find(|player_campaign| player_campaign.campaign_id == new_campaign_id).is_none() {
                player.campaigns.push(PlayerCampaign {
                    campaign_id: new_campaign_id,
                    name: self.name.clone(),
                    is_storyteller: true,
                    characters: Default::default(),
                    dice_channel: self.dice_channel,
                    channels: self.channels.clone(),
                });
            }
            let query = doc! {
                "discordId": &storyteller_bson
            };
            users.replace_one_with_session(query, player, None, session).await?;
        } else {
            // Create a new user with this campaign.
            let new_user = CreateUser {
                version: UserVersion::V0,
                discord_id: self.storyteller,
                campaigns: vec![PlayerCampaign {
                    campaign_id: new_campaign_id,
                    name: self.name.clone(),
                    is_storyteller: true,
                    characters: Default::default(),
                    dice_channel: self.dice_channel,
                    channels: self.channels.clone(),
                }]
            };
            let users = database.collection::<CreateUser>("users");
            users.insert_one_with_session(new_user, None, session).await?;
        }

        // Create channel documents for all of the channels in this campaign
        let mut new_channels = Vec::new();

        for channel in self.channels.iter() {
            let new_channel = CreateChannel {
                version: ChannelVersion::V0,
                channel_id: *channel,
                campaign_id: new_campaign_id.clone(),
            };
            new_channels.push(new_channel);
        }
        let channels = database.collection::<CreateChannel>("channels");
        channels.insert_many_with_session(new_channels, None, session).await?;

        // Transaction complete, commit it
        session.commit_transaction().await?;

        Ok(new_campaign_id)
    }
}
