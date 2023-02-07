use bson::{doc, oid::ObjectId};
use daiklave_core::CharacterMemo as Character;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, UserId};

use crate::{
    error::DocumentError,
    user::{UserCurrent, UserDocument},
};

/// A document to insert a new character into MongoDb.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "character")]
#[serde(rename_all = "camelCase")]
pub struct NewCharacter {
    /// The version of the Character document to be inserted.
    pub version: String,
    /// The Discord snowflake for this character's player.
    pub player: UserId,
    /// The database Id for the campaign this character belongs to.
    pub campaign_id: ObjectId,
    /// The name of the campaign this character belongs to.
    pub campaign_name: String,
    /// The dice channel for the campaign this character belongs to.
    pub dice_channel: ChannelId,
    /// The current Character struct from daiklave-core.
    pub character: Character,
}

impl NewCharacter {
    /// Creates a new Character for a player. This requires a session transaction
    /// because it requires overwriting the player as well as the character.
    pub async fn create(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<ObjectId, crate::error::DocumentError> {
        // Start the transaction
        session.start_transaction(None).await?;

        // Create the character and get the created OID
        let characters = database.collection::<NewCharacter>("characters");
        let InsertOneResult { inserted_id, .. } = characters
            .insert_one_with_session(self, None, session)
            .await?;
        let character_oid = inserted_id
            .as_object_id()
            .ok_or(DocumentError::DeserializationError)?;

        // Get the player with the relevant discord snowflake
        let users = database.collection::<UserDocument>("users");
        let filter = doc! {
            "discordId": bson::to_bson(&self.player).or(Err(DocumentError::SerializationError))?
        };
        let old_player = users
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or(DocumentError::NotFound)?;

        // Add this character's OID and name to their characters list
        let mut new_player: UserCurrent = old_player.clone().into();
        let player_campaign = new_player
            .campaigns
            .get_mut(&self.campaign_id)
            .ok_or(DocumentError::NotFound)?;
        player_campaign
            .characters
            .character
            .insert(character_oid.clone(), self.character.name.clone());

        // Replace that user with the updated document
        users
            .replace_one_with_session(
                bson::to_document(&old_player).or(Err(DocumentError::SerializationError))?,
                &UserDocument::from(new_player),
                None,
                session,
            )
            .await?;

        // Commit transaction and return created OID
        session.commit_transaction().await?;
        Ok(character_oid)
    }
}
