use mongodb::bson::{oid::ObjectId, doc};

use crate::{shared::error::DatabaseError, mongo::{campaigns::CampaignCurrent, users::UserCurrent}};

/// An instruction to rename a campaign.
pub struct RenameCampaign {
    /// The Id of the campaign to rename.
    pub campaign_id: ObjectId,
    /// The new name of the campaign.
    pub name: String,
}

impl RenameCampaign {
    /// Updates MongoDb to have the new name for this campaign.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<(), DatabaseError> {
        session.start_transaction(None).await?;

        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let query = doc! {
            "_id": &self.campaign_id
        };
        let update = doc! {
            "$set": {
                "name": &self.name
            }
        };
        let update_result = campaigns.update_one_with_session(query, update, None, session).await?;
        if update_result.matched_count < 1 {
            return Err(DatabaseError::NotFound(format!("campaign {}", &self.campaign_id)));
        }

        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "campaigns": {
                "campaignId": &self.campaign_id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.name": &self.name
            }
        };
        users.update_many_with_session(query, update, None, session).await?;

        session.commit_transaction().await?;
        Ok(())
    }
}