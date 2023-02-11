use mongodb::bson::{oid::ObjectId, doc, self};
use serenity::all::UserId;

use crate::{shared::error::DataError, mongo::users::UserCurrent};

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
        let user_id_bson = bson::to_bson(&self.user_id).or_else(|_| Err(DataError::SerializationError(format!("{:?}", self.user_id))))?;
        let filter = doc! {
            "discordId": user_id_bson,
            "campaigns": {
                "campaignId": self.campaign_id
            }
        };

        let user = if let Some(user) = users
        .find_one(filter, None)
        .await? {
            user
        } else {
            // The database successfully returned no user with this Id and campaign
            return Ok(None);
        };

        let campaign = if let Some(campaign) = user.campaigns.iter().find(|player_campaign| player_campaign.campaign_id == self.campaign_id) {
            campaign
        } else {
            // This shouldn't happen, but if we get a user that doesn't have this campaign treat them as unauthorized
            return Ok(None);
        };

        Ok(Some(Authorization {
            user_id: self.user_id,
            campaign_id: campaign.campaign_id,
            is_storyteller: campaign.is_storyteller
        }))
    }

    async fn execute_redis<CON: redis::ConnectionLike>(
        &self,
        connection: &mut CON,
    ) -> Result<Option<Authorization>, redis::RedisError> {
        todo!()
    }

    /// Executes a cache-aside lookup; try to retrieve from Redis, fall back to
    /// MongoDb on a cache miss, and populate cache if found.
    pub async fn execute<CON: redis::ConnectionLike>(
        &self,
        database: &mongodb::Database,
        connection: &mut CON,
    ) -> Result<Option<Authorization>, DataError> {
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
