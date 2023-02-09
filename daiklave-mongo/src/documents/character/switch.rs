use bson::{oid::ObjectId, doc};
use serenity::all::UserId;

use crate::{error::DocumentError, user::UserCurrent};

/// An instruction to switch the active character for a campaign.
pub struct SwitchCharacter {
    /// The Discord snowflake of the user switching active characters.
    pub player_id: UserId,
    /// The campaign to switch active characters for.
    pub campaign_id: ObjectId,
    /// The character to make the active character.
    pub character_id: ObjectId,
}

impl SwitchCharacter {
    /// Switches the characters atomically.
    pub async fn execute(&self, database: &mongodb::Database) -> Result<(), DocumentError> {
        let users = database.collection::<UserCurrent>("users");
        let player_bson = bson::to_bson(&self.player_id).or(Err(DocumentError::SerializationError))?;
        let query = doc! {
            "discordId": player_bson,
            "campaigns": {
                "campaignId": self.campaign_id,
                "characters.character.characterId": self.character_id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.characters.active": Some(self.character_id)
            }
        };
        let update_result = users.update_one(query, update, None).await?;
        if update_result.matched_count < 1 {
            Err(DocumentError::NotFound)
        } else {
            Ok(())
        }
    }
}