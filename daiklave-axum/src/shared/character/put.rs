use daiklave_core::CharacterMemo;
use mongodb::{bson::oid::ObjectId, ClientSession};
use redis::AsyncCommands;
use serenity::all::UserId;

use crate::shared::error::DatabaseError;

/// Fully replaces an exiting character. This is mostly used from the Yew
/// frontend to bundle multiple operations together.
pub struct PutCharacter {
    /// The Discord snowflake of the character's player.
    pub player: UserId,
    /// The Id of the campaign to which the character belongs.
    pub campaign_id: ObjectId,
    /// The Id of the character.
    pub character_id: ObjectId,
    /// The replacement character.
    pub character: CharacterMemo,
}

impl PutCharacter {
    async fn execute_mongo(&self,
        _database: &mongodb::Database, 
        _session: &mut ClientSession) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn execute_redis<CON: AsyncCommands>(
        &self,
        _connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        todo!()
    }

    /// Overwrites the existing character in the database, updates the name of
    /// the character in the player's subdocument, and invalidates the cache.
    pub async fn execute<CON: AsyncCommands>(
        &self, 
        database: &mongodb::Database, 
        session: &mut ClientSession,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        self.execute_mongo(database, session).await?;
        self.execute_redis(connection).await?;
        Ok(())
    }
}