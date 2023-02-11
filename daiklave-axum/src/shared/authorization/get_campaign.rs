use mongodb::bson::oid::ObjectId;
use serenity::all::UserId;

use crate::shared::error::DataError;

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
    ) -> Result<Option<Authorization>, mongodb::error::Error> {
        todo!()
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
            Err(mongo_err) => {
                // Can't access source-of-truth, throw 500 or equivalent message to Discord
                return Err(DataError::MongoDb(mongo_err));
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
