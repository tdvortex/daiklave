use mongodb::bson::doc;
use serenity::all::UserId;

use crate::{
    mongo::users::{PlayerCampaign, UserCurrent},
    shared::{error::DatabaseError, to_bson},
};

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
    pub async fn execute(
        &self,
        database: &mongodb::Database,
    ) -> Result<Vec<PlayerCampaign>, DatabaseError> {
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": to_bson(&self.user_id)?,
        };
        Ok(users
            .find_one(filter, None)
            .await?
            .map_or(Vec::new(), |user| user.campaigns))
    }
}
