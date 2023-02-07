use std::collections::HashSet;

use bson::{doc, oid::ObjectId, Bson};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, UserId};

use crate::{
    channel::{ChannelDocument, NewChannel, ChannelVersion},
    error::DocumentError,
    user::{NewUser, UserCurrent, UserDocument, UserVersion},
    PlayerCampaign,
};

use super::versions::CampaignVersion;

/// A Campaign document to be inserted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "campaign")]
#[serde(rename_all = "camelCase")]
pub struct NewCampaign {
    /// The version of the Campaign document to be inserted.
    pub version: CampaignVersion,
    /// The human-readable name of the campaign.
    pub name: String,
    /// The Discord Snowflake representing the storyteller for the campaign.
    pub storyteller: UserId,
    /// All the Discord snowflakes for the players in the
    /// campaign (including the storyteller).
    pub players: HashSet<UserId>,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice
    /// channel)
    pub channels: HashSet<ChannelId>,
}

impl NewCampaign {
    /// Create a new campaign. Requires ClientSession, as this involves
    /// modifying all of the Users in the campaign simultaneously.
    pub async fn create(
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
        let channels = database.collection::<ChannelDocument>("channels");
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

        // Get a handle to the "campaigns" collection, but serializing document using NewCampaign
        let campaigns = database.collection::<NewCampaign>("campaigns");

        // Create a new document for this campaign
        let mongodb::results::InsertOneResult { inserted_id, .. } = campaigns
            .insert_one_with_session(self, None, session)
            .await?;
        let new_campaign_id = inserted_id
            .as_object_id()
            .ok_or(DocumentError::DeserializationError)?;

        // Get documents for all the players of this campaign
        let users = database.collection::<UserDocument>("users");
        let user_ids = self
            .players
            .iter()
            .map(|discord_id| bson::to_bson(discord_id).unwrap_or(Bson::Int32(-1)))
            .collect::<Vec<Bson>>();
        let filter = doc! {
            "discordId" : {
                "$in": user_ids
            }
        };
        let mut existing_users = users
            .find_with_session(filter, None, session)
            .await?
            .stream(session)
            .try_collect::<Vec<UserDocument>>()
            .await?
            .into_iter()
            .map(|user_document| UserCurrent::from(user_document))
            .collect::<Vec<UserCurrent>>();
        let existing_user_ids = existing_users
            .iter()
            .map(|user| user.discord_id)
            .collect::<HashSet<UserId>>();

        // Create documents for any users that don't already have documents
        // Check to see if there are any
        let mut insert_ids = Vec::new();
        for snowflake in self.players.iter() {
            if !existing_user_ids.contains(snowflake) {
                insert_ids.push(*snowflake);
            }
        }

        if !insert_ids.is_empty() {
            // Make a NewUser for each missing Id
            let insert_users = insert_ids.iter().map(|user_id| NewUser {
                version: UserVersion::V0,
                discord_id: *user_id,
            });

            // Insert all of them at once
            let new_users = database.collection::<NewUser>("users");
            new_users
                .insert_many_with_session(insert_users, None, session)
                .await?;

            // Retrieve just the newly inserted UserCurrent documents
            let new_ids_bson = insert_ids
                .iter()
                .map(|discord_id| bson::to_bson(discord_id).unwrap_or(Bson::Int32(-1)))
                .collect::<Vec<Bson>>();
            let filter = doc! {
                "discordId" : {
                    "$in": new_ids_bson
                }
            };
            let created_users = users
                .find(filter, None)
                .await?
                .try_collect::<Vec<UserDocument>>()
                .await?
                .into_iter()
                .map(|user_document| UserCurrent::from(user_document))
                .collect::<Vec<UserCurrent>>();

            // Add those to our existing users vec
            existing_users.extend(created_users.into_iter());
        }

        // Update all the players in the campaign to be a part of it
        // There is no "replace many" option in MongoDb, just chain replace_one
        for old_user in existing_users.into_iter() {
            let mut new_user = old_user.clone();
            let player_campaign = PlayerCampaign {
                campaign_id: new_campaign_id.clone(),
                name: self.name.clone(),
                is_storyteller: self.storyteller == new_user.discord_id,
                characters: Default::default(),
                dice_channel: self.dice_channel,
                channels: self.channels.clone(),
            };
            new_user
                .campaigns
                .push(player_campaign);

            users
                .replace_one_with_session(
                    bson::to_document(&UserDocument::from(old_user))
                        .or(Err(DocumentError::SerializationError))?,
                    &UserDocument::from(new_user),
                    None,
                    session,
                )
                .await?;
        }

        // Create channel documents for all of the channels in this campaign
        let mut new_channels = Vec::new();

        for channel in self.channels.iter() {
            let new_channel = NewChannel {
                version: ChannelVersion::V0,
                channel_id: *channel,
                campaign_id: new_campaign_id.clone(),
            };
            new_channels.push(new_channel);
        }
        let channels = database.collection::<NewChannel>("channels");
        channels.insert_many_with_session(new_channels, None, session).await?;

        // Transaction complete, commit it
        session.commit_transaction().await?;

        Ok(new_campaign_id)
    }
}
