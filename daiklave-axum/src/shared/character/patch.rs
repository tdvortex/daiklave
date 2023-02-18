use daiklave_core::{CharacterMutation, CharacterMemo};
use mongodb::bson::{oid::ObjectId, doc};
use redis::AsyncCommands;
use serenity::all::UserId;

use crate::{mongo::characters::{CharacterCurrent, CharacterV0}, shared::{error::DatabaseError, to_bson}};

/// An instruction to partially update a character using a [CharacterMutation].
pub struct PatchCharacter {
    /// The Discord snowflake of the character's player.
    pub player: UserId,
    /// The Id of the campaign to which the character belongs.
    pub campaign_id: ObjectId,
    /// The Id of the character.
    pub character_id: ObjectId,
    /// The mutation to apply.
    pub mutation: CharacterMutation,
}

impl PatchCharacter {
    /// Starts a session to read, update, and overwrite character. This is 
    /// required on a cache miss, as well as when the mutation changes the 
    /// character's name.
    async fn execute_mongo_session(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<CharacterCurrent, DatabaseError> {
        session.start_transaction(None).await?;

        // Get the existing character
        let characters = database.collection::<CharacterCurrent>("characters");
        let filter = doc! {
            "_id": self.character_id,
            "player": to_bson(&self.player)?,
            "campaignId": self.character_id,
        };
        let CharacterV0 {
            _id,
            player,
            campaign_id,
            character,
        } = characters.find_one_with_session(filter, None, session).await?.ok_or_else(|| DatabaseError::NotFound("Character".to_string()))?;

        session.commit_transaction().await?;
        todo!()
    }

    /// Atomically replaces the character in the database. 
    async fn execute_mongo_replacement(&self, replacement: CharacterMemo, database: &mongodb::Database) -> Result<(), DatabaseError> {
        todo!()
    }

    /// Attempts to get the character from the cache.
    async fn execute_redis_get<CON: AsyncCommands>(&self, connection: &mut CON) -> Result<Option<CharacterCurrent>, DatabaseError> {
        todo!()
    }

    /// Replaces the cached character after an update.
    async fn execute_redis_set<CON: AsyncCommands>(&self, value: CharacterCurrent, connection: &mut CON) -> Result<(), DatabaseError> {
        todo!()
    }

    /// Executes the patch, also updating the cache. Depending on the mutation 
    /// and cache state, this may or may not require a session.
    pub async fn execute<CON: AsyncCommands>(&self, client: &mongodb::Client, database_name: &str, connection: &mut CON) -> Result<(), DatabaseError> {
        let cached_character = if let CharacterMutation::SetName(_) = self.mutation {
            None
        } else {
            self.execute_redis_get(connection).await?
        };

        let patched_character = if let Some(character_current) = cached_character {
            todo!()
        } else {
            todo!()
        };

        // self.execute_redis_set(patched_character, connection).await?;


        Ok(())
    }
}