use daiklave_core::CharacterMemo;
use mongodb::{bson::{oid::ObjectId, doc}, results::InsertOneResult};
use serenity::all::UserId;

use crate::{shared::{error::DatabaseError, to_bson}, mongo::{characters::{InsertCharacter, CharacterVersion}, users::{UserCurrent, CharacterStub}}};

/// The information needed to process a request to create a new character document.
pub struct CreateCharacter {
    /// The Discord snowflake of the character's player.
    pub player: UserId,
    /// The Id of the campaign to which the character belongs.
    pub campaign_id: ObjectId,
    /// The replacement character.
    pub character: CharacterMemo,
}

impl CreateCharacter {
    /// Creates the new character document in MongoDb. Requires a session to 
    /// record this character on the player as well. Does not update Redis 
    /// at this time (lazy-loading).
    pub async fn execute(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<ObjectId, DatabaseError> {
        session.start_transaction(None).await?;

        let character_name = self.character.name.clone();

        let characters = database.collection::<InsertCharacter>("characters");
        let insert_character = InsertCharacter {
            version: CharacterVersion::V0,
            player: self.player,
            campaign_id: self.campaign_id,
            character: self.character.clone(),
        };
        let InsertOneResult {
            inserted_id,
            ..
        } = characters.insert_one_with_session(&insert_character, None, session).await?;

        let inserted_id = inserted_id.as_object_id().ok_or_else(|| DatabaseError::DeserializationError(inserted_id.to_string()))?;

        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "discordId": to_bson(&self.player)?,
            "campaigns": {
                "campaignId": self.campaign_id,
            }
        };
        let update = doc! {
            "$push": {
                "campaigns.$.characters.character": to_bson(&CharacterStub { character_id: inserted_id, name: character_name })?,
            }
        };
        users.update_one_with_session(query, update, None, session).await?;

        session.commit_transaction().await?;
        Ok(inserted_id)
    }
}