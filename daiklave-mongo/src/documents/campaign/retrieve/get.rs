use bson::{oid::ObjectId, doc, Bson};
use serenity::all::UserId;

use crate::{campaign::CampaignCurrent, error::DocumentError};

/// Get a campaign by its Id, for a specific user. If a player is not part of
/// the campaign, they get a 404.
pub struct GetCampaign {
    /// The MongoDB database ID for the campaign.
    pub _id: ObjectId,
    /// The Discord snowflake for the user making the request.
    pub user_id: UserId,
}

impl GetCampaign {
    /// Finds the campaign. No session required, the operation is atomic.
    pub async fn execute(&self, database: &mongodb::Database) -> Result<CampaignCurrent, DocumentError> {
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self._id,
            "players": bson::to_bson(&self.user_id).unwrap_or(Bson::Int32(-1)),
        };
        campaigns.find_one(filter, None).await?.ok_or(DocumentError::NotFound)
    }
}