use bson::{oid::ObjectId, doc};
use mongodb::ClientSession;
use serenity::all::UserId;

use crate::{error::DocumentError, user::UserCurrent};

use super::CharacterCurrent;

/// An instruction to delete a character from a player and their campaign.
pub struct DeleteCharacter {
    /// The player deleting their character.
    pub player: UserId,
    /// The character to be deleted.
    pub character_id: ObjectId,
}

impl DeleteCharacter {
    /// Deletes the character. Requires a session to update the character and player simultaneously.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut ClientSession) -> Result<(), DocumentError> {
        session.start_transaction(None).await?;

        // Delete the character document
        let characters = database.collection::<CharacterCurrent>("characters");
        let player_bson = bson::to_bson(&self.player).or(Err(DocumentError::SerializationError))?;
        let query = doc! {
            "_id": self.character_id,
            "player": &player_bson
        };
        characters.delete_one_with_session(query, None, session).await?;

        // Get the player document
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": &player_bson
        };
        let mut user = users.find_one_with_session(filter, None, session).await?.ok_or(DocumentError::NotFound)?;

        // Modify the player document 
        let campaign = user.campaigns.iter_mut().find(|player_campaign| player_campaign.characters.character.iter().any(|stub| stub.character_id == self.character_id)).ok_or(DocumentError::NotFound)?;
        if let Some(active_id) = campaign.characters.active_character {
            if active_id == self.character_id {
                campaign.characters.active_character = None;
            }
        }
        campaign.characters.character.retain(|stub| stub.character_id != self.character_id);

        // Replace the player document 
        let query = doc! {
            "discordId": player_bson
        };
        users.replace_one_with_session(query, user, None, session).await?;

        session.commit_transaction().await?;
        Ok(())
    }
}