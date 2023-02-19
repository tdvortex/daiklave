use mongodb::bson::{doc, oid::ObjectId};
use serenity::all::UserId;

use crate::{
    mongo::users::{PlayerCampaign, UserCurrent},
    shared::{error::DatabaseError, to_bson},
};

/// An instruction to retrieve the PlayerCampaign subdocument for a player and
/// campaign.
pub struct GetCampaign {
    /// The user making the request.
    pub user_id: UserId,
    /// The Id of the desired campaign.
    pub campaign_id: ObjectId,
}

impl GetCampaign {
    /// Executes the get request against the database. This is not cached to
    /// avoid conflict with permissions caching.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
    ) -> Result<Option<PlayerCampaign>, DatabaseError> {
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": to_bson(&self.user_id)?,
        };
        Ok(users.find_one(filter, None).await?.and_then(|player| {
            player
                .campaigns
                .into_iter()
                .find(|player_campaign| player_campaign.campaign_id == self.campaign_id)
        }))
    }
}
