use bson::{oid::ObjectId, doc};

use crate::{campaign::CampaignCurrent, error::DocumentError};

/// Get a campaign by its Id.
pub struct GetCampaign {
    /// The MongoDB database ID for the campaign.
    pub _id: ObjectId,
}

impl GetCampaign {
    /// Finds the campaign. No session required, the operation is atomic.
    pub async fn execute(&self, database: &mongodb::Database) -> Result<CampaignCurrent, DocumentError> {
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self._id,
        };
        campaigns.find_one(filter, None).await?.ok_or(DocumentError::NotFound)
    }
}