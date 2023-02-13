use mongodb::bson::{doc, oid::ObjectId};
use redis::AsyncCommands;
use serenity::all::UserId;

use crate::{
    mongo::{characters::CharacterCurrent, users::UserCurrent},
    shared::{error::DatabaseError, to_bson},
};

/// An instruction to delete a character.
pub struct DeleteCharacter {
    /// The Discord snowflake of the character's player.
    pub player: UserId,
    /// The Id of the campaign to which the character belongs.
    pub campaign_id: ObjectId,
    /// The Id of the character.
    pub character_id: ObjectId,
}

impl DeleteCharacter {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<(), DatabaseError> {
        session.start_transaction(None).await?;

        let player_bson = to_bson(&self.player)?;

        let characters = database.collection::<CharacterCurrent>("characters");
        let query = doc! {
            "_id": self.character_id,
            "player": &player_bson,
            "campaignId": self.character_id,
        };
        let delete_result = characters
            .delete_one_with_session(query, None, session)
            .await?;
        if delete_result.deleted_count < 1 {
            return Err(DatabaseError::NotFound("Character".to_string()));
        }

        // Get the player document
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": &player_bson
        };
        let mut user = users
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or_else(|| DatabaseError::NotFound("User".to_string()))?;

        // Modify the player document
        let campaign = user
            .campaigns
            .iter_mut()
            .find(|player_campaign| {
                player_campaign.campaign_id == self.campaign_id
                    && player_campaign
                        .characters
                        .character
                        .iter()
                        .any(|stub| stub.character_id == self.character_id)
            })
            .ok_or_else(|| DatabaseError::NotFound("PlayerCharacter".to_string()))?;
        if let Some(active_id) = campaign.characters.active_character {
            if active_id == self.character_id {
                campaign.characters.active_character = None;
            }
        }
        campaign
            .characters
            .character
            .retain(|stub| stub.character_id != self.character_id);

        // Replace the player document
        let query = doc! {
            "discordId": player_bson
        };
        users
            .replace_one_with_session(query, user, None, session)
            .await?;

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

    /// Delete the character from the database and invalidate the cache.
    pub async fn execute<CON: AsyncCommands>(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        self.execute_mongo(database, session).await?;
        self.execute_redis(connection).await?;
        Ok(())
    }
}
