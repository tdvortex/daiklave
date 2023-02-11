use mongodb::bson::{doc, self};
use serenity::all::UserId;

use crate::{shared::error::DataError, mongo::users::{PlayerCampaign, UserCurrent}};

/// A data-layer request to get all of the campaigns the player is a part of,
/// as well as the names and Ids of characters in those campaigns. 
/// This request does *not* require prior authentication; only campaigns the 
/// user is authenticated to see are shown. This is not cached server-side.
pub struct ListCampaigns {
    /// The user making the request.
    pub user_id: UserId,
}

impl ListCampaigns {
    /// Retrieves the user's PlayerCampaign list. Returns an empty list
    /// if the user has no campaigns.
    pub async fn execute(&self, database: &mongodb::Database) -> Result<Vec<PlayerCampaign>, DataError> {
        let users = database.collection::<UserCurrent>("users");
        let user_id_bson = bson::to_bson(&self.user_id)
            .or_else(|_| Err(DataError::SerializationError(format!("{:?}", self.user_id))))?;
        let filter = doc! {
            "discordId": user_id_bson
        };
        Ok(users.find_one(filter, None).await?.map_or(Vec::new(), |user| user.campaigns))
    }
}