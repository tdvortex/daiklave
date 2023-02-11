use mongodb::bson::{self, doc, oid::ObjectId};
use serenity::all::UserId;

use crate::{mongo::users::UserCurrent, shared::error::DataError};

use super::Authorization;

/// A data-layer request to return the authorization status of a user with
/// regards to a specific campaign. This is mostly used from the HTTP API
/// endpoint where a campaign Id must typically be specified in the URL,
/// like GET /api/campaigns/:campaignId/charms/:charmId
pub struct GetCampaignAuthorization {
    /// The Discord snowflake of the user making the request
    pub user_id: UserId,
    /// The campaign Id the user is making a request for
    pub campaign_id: ObjectId,
}

impl GetCampaignAuthorization {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
    ) -> Result<Option<Authorization>, DataError> {
        let users = database.collection::<UserCurrent>("users");
        let user_id_bson = bson::to_bson(&self.user_id)
            .or_else(|_| Err(DataError::SerializationError(format!("{:?}", self.user_id))))?;
        let filter = doc! {
            "discordId": user_id_bson,
            "campaigns": {
                "campaignId": self.campaign_id
            }
        };

        let user = if let Some(user) = users.find_one(filter, None).await? {
            user
        } else {
            // The database successfully returned no user with this Id and campaign
            return Ok(None);
        };

        let campaign = if let Some(campaign) = user
            .campaigns
            .iter()
            .find(|player_campaign| player_campaign.campaign_id == self.campaign_id)
        {
            campaign
        } else {
            // This shouldn't happen, but if we get a user that doesn't have this campaign treat them as unauthorized
            return Ok(None);
        };

        Ok(Some(Authorization {
            user_id: self.user_id,
            campaign_id: campaign.campaign_id,
            is_storyteller: campaign.is_storyteller,
        }))
    }

    async fn execute_redis<CON>(
        &self,
        connection: &mut CON,
    ) -> Result<Option<Authorization>, redis::RedisError>
    where
        CON: redis::AsyncCommands,
    {
        let mut key = "userId:".as_bytes().to_vec();
        key.extend(self.user_id.0.get().to_be_bytes());

        let mut field = "campaignId:".as_bytes().to_vec();
        field.extend(self.campaign_id.bytes());

        let maybe_is_storyteller: Option<bool> = connection.hget(key, field).await?;
        if let Some(is_storyteller) = maybe_is_storyteller {
            Ok(Some(Authorization {
                user_id: self.user_id,
                campaign_id: self.campaign_id,
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

        // Either a connection error with Redis or a cache miss, try to get from Mongo
        let maybe_authorization = match self.execute_mongo(database).await {
            Ok(maybe_authorization) => maybe_authorization,
            Err(e) => {
                // Can't access source-of-truth, throw 500 or equivalent message to Discord
                return Err(e);
            }
        };

        if let Some(authorization) = maybe_authorization {
            // Found authorization in MongoDb; backfill the cache
            // TODO
            Ok(Some(authorization))
        } else {
            // The user is not authorized for this campaign
            Ok(None)
        }
    }
}
