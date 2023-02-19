use std::collections::HashSet;

use mongodb::bson::{doc, oid::ObjectId};
use redis::AsyncCommands;
use serenity::all::{ChannelId, UserId};

use crate::{
    mongo::{
        campaigns::CampaignCurrent,
        channels::{ChannelCurrent, ChannelVersion, InsertChannel},
        users::UserCurrent,
    },
    shared::{
        error::{ConstraintError, DatabaseError},
        to_bson,
    },
};

/// An instruction to replace the channels set for the campaign.
pub struct SetCampaignChannels {
    /// Th Id of the campaign to update
    pub campaign_id: ObjectId,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice
    /// channel)
    pub channels: HashSet<ChannelId>,
}

struct ChannelUpdate {
    users: HashSet<UserId>,
    removed_channels: Vec<ChannelId>,
}

impl SetCampaignChannels {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<ChannelUpdate, DatabaseError> {
        session.start_transaction(None).await?;

        // Get the existing campaign document
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": &self.campaign_id
        };
        let old_campaign = campaigns
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or_else(|| DatabaseError::NotFound("Campaign".to_owned()))?;

        // Get the campaign's existing details and make a new version
        let mut new_campaign = old_campaign.clone();
        new_campaign.dice_channel = self.dice_channel;
        new_campaign.channels = self.channels.clone();

        // Determine the marginal changes
        let channel_update = ChannelUpdate {
            users: old_campaign.players,
            removed_channels: old_campaign
                .channels
                .difference(&new_campaign.channels)
                .copied()
                .collect(),
        };
        let added_channels: Vec<ChannelId> = new_campaign
            .channels
            .difference(&old_campaign.channels)
            .copied()
            .collect();

        // Check to make sure the added channels aren't currently in use
        let channels = database.collection::<ChannelCurrent>("channels");
        let filter = doc! {
            "channelId": {
                "$in": to_bson(&added_channels)?,
            },
            "campaignId": &self.campaign_id,
        };
        if let Some(channel) = channels
            .find_one_with_session(filter, None, session)
            .await?
        {
            return Err(DatabaseError::ConstraintError(
                ConstraintError::ChannelCampaignUnique(channel.channel_id),
            ));
        }

        // Remove all of the removed channels
        let query = doc! {
            "channelId": {
                "$in": to_bson(&channel_update.removed_channels)?,
            },
            "campaignId": &self.campaign_id,
        };
        channels
            .delete_many_with_session(query, None, session)
            .await?;

        // Add all of the added channels
        let mut new_channels = Vec::new();
        for channel in added_channels.iter() {
            let new_channel = InsertChannel {
                version: ChannelVersion::V0,
                channel_id: *channel,
                campaign_id: self.campaign_id,
            };
            new_channels.push(new_channel);
        }
        let channels = database.collection::<InsertChannel>("channels");
        channels
            .insert_many_with_session(new_channels, None, session)
            .await?;

        // Replace the campaign document
        let query = doc! {
            "_id": &self.campaign_id
        };
        campaigns
            .replace_one_with_session(query, new_campaign, None, session)
            .await?;

        // Update all players in this campaign to have the correct channels
        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "campaigns": {
                "campaignId": self.campaign_id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.diceChannel": to_bson(&self.dice_channel)?,
                "campaigns.$.channels": to_bson(&self.channels)?,
            }
        };
        users
            .update_many_with_session(query, update, None, session)
            .await?;

        session.commit_transaction().await?;

        return Ok(channel_update);
    }

    /// Invalidate the cache for all removed channels
    /// Added channels will be lazy-loaded on future cache misses
    async fn execute_redis<CON: AsyncCommands>(
        &self,
        update: &ChannelUpdate,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        for &user_id in update.users.iter() {
            // Get the key for this user's Redis hash
            let mut key = "userId:".as_bytes().to_vec();
            key.extend(user_id.0.get().to_be_bytes());

            // Get the fields for all of the removed channels
            let mut delete_fields = Vec::new();
            for &channel in update.removed_channels.iter() {
                let mut channel_field = "channelId:".as_bytes().to_vec();
                channel_field.extend(channel.0.get().to_be_bytes());

                // Push NOT extend. We want a Vec<Vec<u8>>.
                delete_fields.push(channel_field);
            }

            // Delete all these channels from the user
            connection.hdel(key, delete_fields).await?;
        }

        Ok(())
    }

    /// Update the database and authorization cache for these channels
    pub async fn execute<CON: AsyncCommands>(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        // Update MongoDb
        let channel_update = self.execute_mongo(database, session).await?;

        // If necessary, remove invalidated channel permissions
        if !channel_update.users.is_empty() && !channel_update.removed_channels.is_empty() {
            self.execute_redis(&channel_update, connection).await?;
        }

        Ok(())
    }
}
