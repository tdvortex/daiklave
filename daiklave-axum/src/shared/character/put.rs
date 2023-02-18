use daiklave_core::CharacterMemo;
use mongodb::{
    bson::{doc, oid::ObjectId},
    ClientSession,
};
use redis::AsyncCommands;
use serenity::all::UserId;

use crate::{
    mongo::{characters::CharacterCurrent, users::UserCurrent},
    shared::{error::DatabaseError, to_bson},
};

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
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
        session: &mut ClientSession,
    ) -> Result<(), DatabaseError> {
        session.start_transaction(None).await?;

        // Replace the character document
        let characters = database.collection::<CharacterCurrent>("characters");
        let query = doc! {
            "_id": self.character_id,
            "player": to_bson(&self.player)?,
            "campaignId": self.character_id,
        };
        let replacement = CharacterCurrent {
            _id: self.character_id,
            player: self.player,
            campaign_id: self.campaign_id,
            character: self.character.clone(),
        };
        let replace_result = characters
            .replace_one_with_session(query, replacement, None, session)
            .await?;
        if replace_result.matched_count < 1 {
            return Err(DatabaseError::NotFound("Character".to_owned()));
        }

        // Update the name in the player document
        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "discordId": to_bson(&self.player)?,
            "campaigns": {
                "campaignId": self.campaign_id,
                "characters.character.characterId": self.character_id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.characters.character.$.name": self.character.name.clone(),
            }
        };
        users
            .update_one_with_session(query, update, None, session)
            .await?;

        // Done with database
        session.commit_transaction().await?;
        Ok(())
    }

    async fn execute_redis<CON: AsyncCommands>(
        &self,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        let mut key = "characterId:".as_bytes().to_vec();
        key.extend(self.character_id.bytes());

        connection.del(vec![key]).await?;
        Ok(())
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
