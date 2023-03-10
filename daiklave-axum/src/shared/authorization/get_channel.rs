use mongodb::bson::doc;
use serenity::all::{ChannelId, UserId};

use crate::{
    mongo::users::UserCurrent,
    shared::{error::DatabaseError, to_bson},
};

use super::{Authorization, CampaignAuthorization, ChannelAuthorization};

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
    ) -> Result<Option<UserCurrent>, DatabaseError> {
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": to_bson(&self.user_id)?,
            "campaigns": {
                "channels": to_bson(&self.channel_id)?,
            }
        };

        Ok(users.find_one(filter, None).await?)
    }

    async fn execute_redis<CON>(
        &self,
        connection: &mut CON,
    ) -> Result<Option<ChannelAuthorization>, DatabaseError>
    where
        CON: redis::AsyncCommands,
    {
        // Key = "userId:" + big-endian bytes of their Discord snowflake
        let mut key = "userId:".as_bytes().to_vec();
        key.extend(self.user_id.0.get().to_be_bytes());

        // Channel field(s) = "channelId" + big-endian bytes of the channel's Discord snowflake
        let mut field = "channelId:".as_bytes().to_vec();
        field.extend(self.channel_id.0.get().to_be_bytes());

        // Channel value(s) = serialized ChannelAuthorization
        let maybe_value_bytes: Option<Vec<u8>> = connection.hget(key, field).await?;
        if let Some(value_bytes) = maybe_value_bytes {
            if let Ok(channel_auth) = postcard::from_bytes(&value_bytes) {
                Ok(Some(channel_auth))
            } else {
                Err(DatabaseError::DeserializationError(
                    "Channel auth".to_owned(),
                ))
            }
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
    ) -> Result<Option<Authorization>, DatabaseError>
    where
        CON: redis::AsyncCommands,
    {
        // Try to get from the cache
        if let Ok(Some(ChannelAuthorization {
            campaign_id,
            active_character,
            is_storyteller,
        })) = self.execute_redis(connection).await
        {
            // Cache hit, return
            return Ok(Some(Authorization {
                user_id: self.user_id,
                campaign_id,
                is_storyteller,
                active_character,
            }));
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
            // Campaign value = Postcart serialization of CampaignAuthorization
            // Channel field(s) = "channelId" + big-endian bytes of the channel's Discord snowflake
            // Channel value(s) = Postcard serialization of ChannelAuthorization
            let is_storyteller = campaign.is_storyteller;

            let mut key = "userId:".as_bytes().to_vec();
            key.extend(self.user_id.0.get().to_be_bytes());

            let mut items = Vec::new();
            let mut campaign_field = "campaignId:".as_bytes().to_vec();
            campaign_field.extend(campaign.campaign_id.bytes());

            let campaign_value =
                postcard::to_allocvec(&CampaignAuthorization { is_storyteller })
                    .map_err(|_| DatabaseError::SerializationError("Campaign Auth".to_owned()))?;
            items.push((campaign_field, campaign_value));

            for channel in campaign.channels.iter() {
                let mut channel_field = "channelId:".as_bytes().to_vec();
                channel_field.extend(channel.0.get().to_be_bytes());

                let channel_value = postcard::to_allocvec(&ChannelAuthorization {
                    campaign_id: campaign.campaign_id,
                    active_character: campaign.characters.active_character,
                    is_storyteller,
                })
                .map_err(|_| DatabaseError::SerializationError("Channel Auth".to_owned()))?;

                items.push((channel_field, channel_value));
            }

            // We don't need the redis result (or even for it to succeed)
            let _: Result<Vec<Vec<u8>>, redis::RedisError> =
                connection.hset_multiple(key, &items).await;

            // Return the completed auth
            Ok(Some(Authorization {
                user_id: self.user_id,
                campaign_id: campaign.campaign_id,
                is_storyteller,
                active_character: campaign.characters.active_character,
            }))
        } else {
            // The user is not authorized for this campaign
            Ok(None)
        }
    }
}
