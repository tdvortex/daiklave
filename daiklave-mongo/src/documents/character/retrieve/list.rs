use bson::{doc, oid::ObjectId};
use serenity::all::UserId;

use crate::{error::DocumentError, user::UserCurrent, PlayerCharacters};

/// An instruction to list stubs of all of the characters the player has.
pub struct ListCharacters {
    /// The Discord snowflake for the player requesting their characters.
    pub player: UserId,
}

impl ListCharacters {
    /// Returns Ok and a vector of (campaign Id, campaign name, player 
    /// character stubs) if the user is found. Returns Err if the player
    /// is not found.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
    ) -> Result<Vec<(ObjectId, String, PlayerCharacters)>, DocumentError> {
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": bson::to_bson(&self.player).or(Err(DocumentError::SerializationError))?
        };
        let user = users
            .find_one(filter, None)
            .await?
            .ok_or(DocumentError::NotFound)?;

        Ok(user
            .campaigns
            .into_iter()
            .map(|player_campaign| {
                (
                    player_campaign.campaign_id,
                    player_campaign.name,
                    player_campaign.characters,
                )
            })
            .collect::<Vec<(ObjectId, String, PlayerCharacters)>>())
    }
}
