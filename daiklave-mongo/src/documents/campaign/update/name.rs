use bson::{oid::ObjectId, doc};
use mongodb::ClientSession;

use crate::{error::DocumentError, campaign::CampaignDocument, character::CharacterDocument, user::UserDocument};

/// Instruction to change the name of a campaign.
pub struct UpdateCampaignName {
    /// The MongoDb OID of the campaign to be renamed.
    pub campaign_id: ObjectId,
    /// The new name of the campaign.
    pub new_name: String,
}

impl UpdateCampaignName {
    /// Changes the name of a campaign. Uses a session to atomically update 
    /// all characters for all players in the campaign.
    pub async fn update(&self, database: &mongodb::Database, session: &mut ClientSession) -> Result<(), DocumentError> {
        session.start_transaction(None).await?;

        // Update the name in the campaign document itself
        let campaigns = database.collection::<CampaignDocument>("campaigns");
        let query = doc!{
            "_id": self.campaign_id
        };
        let update = doc!{
            "$set": {
                "name": &self.new_name
            }
        };
        campaigns.update_one_with_session(query, update, None, session).await?;

        // Update the name of the campaign in all characters
        let characters = database.collection::<CharacterDocument>("characters");
        let query = doc!{
            "campaignId": self.campaign_id
        };
        let update = doc!{
            "$set": {
                "campaignName": &self.new_name
            }
        };
        characters.update_many_with_session(query, update, None, session).await?;

        // Update the name of the campaign for all players
        let users = database.collection::<UserDocument>("users");
        let query = doc!{
            "campaigns": {
                "campaignId": self.campaign_id
            }
        };
        let update = doc!{
            "$set": {
                "campaigns.$[elem].name" : &self.new_name
            }
        };
        let array_filter = doc!{
            "elem.campaignId": self.campaign_id
        };
        let options = mongodb::options::UpdateOptions::builder().array_filters(Some(vec![array_filter])).build();
        users.update_many_with_session(query, update, options, session).await?;
        

        session.commit_transaction().await?;
        Ok(())
    }
}