
use bson::{oid::ObjectId, doc};
use daiklave_core::CharacterMemo as Character;
use serenity::all::UserId;

use crate::{error::DocumentError, user::UserCurrent};

use super::CharacterCurrent;

/// An instruction to update a character to a certain state. This also makes 
/// the character the "active" character for this player for this campaign.
pub struct UpdateCharacter {
    /// The MongoDb database Id for this character.
    pub _id: ObjectId,
    /// The Discord snowflake for this character's player.
    pub player: UserId,
    /// The database Id for the campaign this character belongs to.
    pub campaign_id: ObjectId,
    /// The current Character struct from daiklave-core.
    pub character: Character,
}

impl UpdateCharacter {
    /// Replaces the character document, and updates the player document to 
    /// reflect this character as "active". Requires a quick transaction to
    /// sync the name/activity of the character with the users collection.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<(), DocumentError> {
        session.start_transaction(None).await?;

        // Replace the character document
        let characters = database.collection::<CharacterCurrent>("characters");
        let query = doc! {
            "_id": self._id
        };
        let replacement = CharacterCurrent {
            _id: self._id,
            player: self.player,
            campaign_id: self.campaign_id,
            character: self.character.clone(),
        };
        characters.replace_one_with_session(query, replacement, None, session).await?;

        // Update the name and activity status in the player document
        let users = database.collection::<UserCurrent>("users");
        let player_bson = bson::to_bson(&self.player).or(Err(DocumentError::SerializationError))?;
        let query = doc! {
            "discordId": player_bson,
            "campaigns": {
                "campaignId": self.campaign_id,
                "characters.character.characterId": self._id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.characters.active": Some(self._id),
                "campaigns.$.characters.character.$.name": self.character.name.clone(),
            }
        };
        users.update_one_with_session(query, update, None, session).await?;

        // Done with database
        session.commit_transaction().await?;
        Ok(())
    }
}