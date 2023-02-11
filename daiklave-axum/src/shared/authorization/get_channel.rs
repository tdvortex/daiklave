use mongodb::bson::{self, doc, oid::ObjectId};
use serenity::all::{ChannelId, UserId};

use crate::{mongo::users::UserCurrent, shared::error::DataError};

use super::Authorization;

/// A data-layer request to return the authorization status of a user with
/// regards to a specific channel. This is mostly used from the Discord
/// endpoint where we receive (user_id, channel_id) but not campaign_id.
pub struct GetChannelAuthorization {
    /// The Discord snowflake of the user making the request
    pub user_id: UserId,
    /// The Discord snowflake of the channel the request was sent from
    pub channel_id: ChannelId,
}

impl GetChannelAuthorization {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
    ) -> Result<Option<UserCurrent>, DataError> {
        let users = database.collection::<UserCurrent>("users");
        let user_id_bson = bson::to_bson(&self.user_id)
            .or_else(|_| Err(DataError::SerializationError(format!("{:?}", self.user_id))))?;
        let channel_id_bson = bson::to_bson(&self.channel_id)
            .or_else(|_| Err(DataError::SerializationError(format!("{:?}", self.channel_id))))?;
        let filter = doc! {
            "discordId": user_id_bson,
            "campaigns": {
                "channels": channel_id_bson
            }
        };

        Ok(users.find_one(filter, None).await?)
    }

    async fn execute_redis<CON>(
        &self,
        connection: &mut CON,
    ) -> Result<Option<Authorization>, redis::RedisError>
    where
        CON: redis::AsyncCommands,
    {
        // Key = "userId:" + big-endian bytes of their Discord snowflake
        let mut key = "userId:".as_bytes().to_vec();
        key.extend(self.user_id.0.get().to_be_bytes());

        // Channel field(s) = "channelId" + big-endian bytes of the channel's Discord snowflake
        let mut field = "channelId:".as_bytes().to_vec();
        field.extend(self.channel_id.0.get().to_be_bytes());

        // Channel value(s) = bytes of the campaign's ObjectId, extended with u8::from(is_storyteller)
        let maybe_value_bytes: Option<Vec<u8>> = connection.hget(key, field).await?;
        if let Some(mut value_bytes) = maybe_value_bytes {
            let is_storyteller = if let Some(byte) = value_bytes.pop() {
                byte == u8::from(true)
            } else {
                // Value bytes is empty, this is technically a deserialization error
                return Ok(None);
            };

            let campaign_id = if value_bytes.len() == 12 {
                ObjectId::from_bytes(value_bytes.into_iter().take(12).enumerate().fold([0; 12], |mut arr, (i, byte)| {arr[i] = byte; arr}))
            } else {
                // Channel value is not 13 bytes long as expected, technically a deserialization error
                return Ok(None);
            };

            Ok(Some(Authorization {
                user_id: self.user_id,
                campaign_id,
                is_storyteller,
            }))
        } else {
            Ok(None)
        }
    }

    /// Executes a cache-aside lookup; try to retrieve from Redis, fall back to
    /// MongoDb on a cache miss, and populate cache if found.
    pub async fn execute<CON>(
        &self,
        database: &mongodb::Database,
        connection: &mut CON,
    ) -> Result<Option<Authorization>, DataError>
    where
        CON: redis::AsyncCommands,
    {
        // Try to get from the cache
        if let Ok(Some(authorization)) = self.execute_redis(connection).await {
            // Cache hit, return
            return Ok(Some(authorization));
        }

        // Either a connection error with Redis or a cache miss, try to get user from Mongo
        let maybe_user = self.execute_mongo(database).await?;

        if let Some(user) = maybe_user {
            // Found the user in MongoDb with this campaign
            // Get the campaign out of the user that has this channel
            let campaign = if let Some(campaign) = user
                .campaigns
                .iter()
                .find(|player_campaign| player_campaign.channels.contains(&self.channel_id))
            {
                campaign
            } else {
                // This shouldn't happen, but if we get a user that doesn't 
                // have a campaign with this channel treat them as unauthorized and don't 
                // update cache
                return Ok(None);
            };

            // Populate the user's permissions hash set in Redis
            // Key = "userId:" + big-endian bytes of their Discord snowflake
            // Campaign field = "campaignId" + bytes of the campaign's ObjectId
            // Campaign value = vec![u8::from(is_storyteller)], to have a consistent data type
            // Channel field(s) = "channelId" + big-endian bytes of the channel's Discord snowflake
            // Channel value(s) = bytes of the campaign's ObjectId, extended with [u8::from(is_storyteller)
            let is_storyteller = campaign.is_storyteller;

            let mut key = "userId:".as_bytes().to_vec();
            key.extend(self.user_id.0.get().to_be_bytes());

            let mut items = Vec::new();
            let mut campaign_field = "campaignId:".as_bytes().to_vec();
            campaign_field.extend(campaign.campaign_id.bytes());

            let campaign_value = vec![u8::from(campaign.is_storyteller)];
            items.push((campaign_field, campaign_value));

            for channel in campaign.channels.iter() {
                let mut channel_field = "channelId:".as_bytes().to_vec();
                channel_field.extend(channel.0.get().to_be_bytes());

                let mut channel_value = campaign.campaign_id.bytes().to_vec();
                channel_value.push(u8::from(is_storyteller));

                items.push((channel_field, channel_value));
            }

            // We don't need the redis result (or even for it to succeed)
            let _: Result<Vec<Vec<u8>>, redis::RedisError> = connection.hset_multiple(key, &items).await;

            // Return the completed auth
            Ok(Some(Authorization { user_id: self.user_id, campaign_id: campaign.campaign_id, is_storyteller }))
        } else {
            // The user is not authorized for this campaign
            Ok(None)
        }
    }
}
