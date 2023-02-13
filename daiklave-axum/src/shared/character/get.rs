use mongodb::bson::{doc, oid::ObjectId};
use redis::AsyncCommands;
use serenity::all::UserId;

use crate::{
    mongo::characters::CharacterCurrent,
    shared::{error::DatabaseError, to_bson},
};

/// An instruction to retrieve a specific character for that player.
pub struct GetCharacter {
    /// The Discord snowflake of the character's player.
    pub player: UserId,
    /// The Id of the campaign to which the character belongs.
    pub campaign_id: ObjectId,
    /// The Id of the character.
    pub character_id: ObjectId,
}

impl GetCharacter {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
    ) -> Result<Option<CharacterCurrent>, DatabaseError> {
        let characters = database.collection("characters");
        let filter = doc! {
            "_id": self.character_id,
            "player": to_bson(&self.player)?,
            "campaignId": self.character_id,
        };
        Ok(characters.find_one(filter, None).await?)
    }

    async fn execute_redis<CON: AsyncCommands>(
        &self,
        connection: &mut CON,
    ) -> Result<Option<CharacterCurrent>, DatabaseError> {
        let mut key = "characterId:".as_bytes().to_vec();
        key.extend(self.character_id.bytes());

        let maybe_bytes: Option<Vec<u8>> = connection.get(vec![key]).await?;

        if let Some(bytes) = maybe_bytes {
            let character: CharacterCurrent = postcard::from_bytes(&bytes)
                .map_err(|_| DatabaseError::DeserializationError("Character".to_owned()))?;

            // Only return the character if the campaign and player match the request
            if character.campaign_id == self.campaign_id && character.player == self.player {
                return Ok(Some(character));
            }
        }

        Ok(None)
    }

    /// Perform a cache-aside lookup to retrieve a specific character, in a 
    /// specific campaign, for that character's player.
    pub async fn execute<CON: AsyncCommands>(
        &self,
        database: &mongodb::Database,
        connection: &mut CON,
    ) -> Result<Option<CharacterCurrent>, DatabaseError> {
        if let Ok(Some(character)) = self.execute_redis(connection).await {
            return Ok(Some(character));
        }

        if let Some(character) = self.execute_mongo(database).await? {
            let mut key = "characterId:".as_bytes().to_vec();
            key.extend(self.character_id.bytes());

            let value = postcard::to_allocvec(&character)
                .map_err(|_| DatabaseError::SerializationError("Character".to_owned()))?;
            let _: Result<Vec<Vec<u8>>, redis::RedisError> = connection.set(key, value).await;

            return Ok(Some(character));
        }

        Ok(None)
    }
}
