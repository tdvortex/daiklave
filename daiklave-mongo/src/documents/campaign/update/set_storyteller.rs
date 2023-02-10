use bson::oid::ObjectId;
use serenity::all::UserId;

use crate::error::DocumentError;

/// Reassigns the storyteller for this campaign. The storyteller document will 
/// be created if it does not exist, and will be added to the campaign if not
/// already a part of it. The previous storyteller (the one invokind this 
/// command) will be left as a player in the campaign.
pub struct SetStoryteller {
    /// The Id of the campaign.
    pub campaign_id: ObjectId,
    /// The Id of the new storyteller.
    pub user_id: UserId,
}

impl SetStoryteller {
    /// Reassigns the storyteller. Requires a session to update campaign and 
    /// players atomically.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<(), DocumentError> {
        todo!()
    }
}