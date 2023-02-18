use daiklave_core::{CharacterMemo, CharacterMutation};
use mongodb::bson::{doc, oid::ObjectId};
use redis::AsyncCommands;
use serenity::all::UserId;

use crate::{
    mongo::{
        characters::{CharacterCurrent},
        users::UserCurrent,
    },
    shared::{
        error::{ConstraintError, DatabaseError},
        to_bson,
    },
};

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
    async fn execute_mongo_session(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<CharacterCurrent, DatabaseError> {
        session.start_transaction(None).await?;

        let player_bson = to_bson(&self.player)?;

        // Get the existing character
        let characters = database.collection::<CharacterCurrent>("characters");
        let filter = doc! {
            "_id": self.character_id,
            "player": &player_bson,
            "campaignId": self.campaign_id,
        };
        let CharacterCurrent {
            _id,
            player,
            campaign_id,
            character,
        } = characters
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or_else(|| DatabaseError::NotFound("Character".to_string()))?;

        let new_character = character
            .apply_mutation(&self.mutation)
            .map_err(|mutation_error| {
                DatabaseError::ConstraintError(ConstraintError::MutationError(mutation_error))
            })?;
        // Check if the name changed
        let name_update = if new_character.name != character.name {
            Some(new_character.name.clone())
        } else {
            None
        };

        let new_character_document = CharacterCurrent {
            _id,
            player,
            campaign_id,
            character: new_character,
        };

        // Replace the character in the database.
        let query = doc! {
            "_id": self.character_id,
            "player": &player_bson,
            "campaignId": self.campaign_id,
        };
        characters
            .replace_one_with_session(query, &new_character_document, None, session)
            .await?;

        // If the name changed, update the player document as well
        if let Some(name) = name_update {
            let users = database.collection::<UserCurrent>("users");
            let query = doc! {
                "discordId": player_bson,
                "campaigns": {
                    "campaignId": self.campaign_id,
                    "characters.character.characterId": self.character_id,
                }
            };
            let update = doc! {
                "$set": {
                    "campaigns.$.characters.character.$.name": name,
                }
            };
            users
                .update_one_with_session(query, update, None, session)
                .await?;
        }

        session.commit_transaction().await?;
        Ok(new_character_document)
    }

    /// Atomically replaces the character in the database. This will not update
    /// the player document.
    async fn execute_mongo_replacement(
        &self,
        replacement: CharacterMemo,
        database: &mongodb::Database,
    ) -> Result<(), DatabaseError> {
        let characters = database.collection::<CharacterCurrent>("characters");
        let new_character_document = CharacterCurrent {
            _id: self.character_id,
            player: self.player,
            campaign_id: self.campaign_id,
            character: replacement,
        };

        // Replace the character in the database.
        let query = doc! {
            "_id": self.character_id,
            "player": to_bson(&self.player)?,
            "campaignId": self.campaign_id,
        };
        characters
            .replace_one(query, new_character_document, None)
            .await?;
        Ok(())
    }

    /// Attempts to get the character from the cache.
    async fn execute_redis_get<CON: AsyncCommands>(
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

    /// Replaces the cached character after an update.
    async fn execute_redis_set<CON: AsyncCommands>(
        &self,
        value: &CharacterCurrent,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        let mut key = "characterId:".as_bytes().to_vec();
        key.extend(value._id.bytes());

        let value = postcard::to_allocvec(&value.character)
            .map_err(|_| DatabaseError::SerializationError("Character".to_owned()))?;
        let _: Result<Vec<Vec<u8>>, redis::RedisError> = connection.set(key, value).await;

        Ok(())
    }

    /// Executes the patch, also updating the cache. Depending on the mutation
    /// and cache state, this may or may not require a session.
    pub async fn execute<CON: AsyncCommands>(
        &self,
        client: &mongodb::Client,
        database_name: &str,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        let cached_character = if let CharacterMutation::SetName(_) = self.mutation {
            // If we have to update the character's name, a multi-collection 
            // transaction is required anyway
            None
        } else {
            self.execute_redis_get(connection).await?
        };

        let patched_character = if let Some(character_current) = cached_character {
            // Update the cached character and update it atomically in MongoDB
            let CharacterCurrent {
                _id,
                player,
                campaign_id,
                character,
            } = character_current;

            let new_character = character.apply_mutation(&self.mutation).map_err(|mutation_error| {
                DatabaseError::ConstraintError(ConstraintError::MutationError(mutation_error))
            })?;

            let database = client.database(database_name);
            self.execute_mongo_replacement(new_character.clone(), &database).await?;
            CharacterCurrent {
                _id,
                player,
                campaign_id,
                character: new_character
            }
        } else {
            // Use a session transaction to retrive and update the character from MongoDb
            let database = client.database(database_name);
            let mut session = client.start_session(None).await?;
            self.execute_mongo_session(&database, &mut session).await?
        };

        // Override the previous cache with the new character
        self.execute_redis_set(&patched_character, connection).await?;

        Ok(())
    }
}
