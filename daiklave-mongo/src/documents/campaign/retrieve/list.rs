use bson::{doc, Bson};
use serenity::all::UserId;

use crate::{error::DocumentError, user::UserCurrent, PlayerCampaign};

/// List all of the campaigns a player is a part of.
pub struct ListCampaigns {
    /// The Discord snowflake of the user making the request.
    pub user_id: UserId,
}

impl ListCampaigns {
    /// Executes the command, returning a Vec of the player's campaigns.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
    ) -> Result<Vec<PlayerCampaign>, DocumentError> {
        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": bson::to_bson(&self.user_id).unwrap_or(Bson::Int32(-1)),
        };
        Ok(users
            .find_one(filter, None)
            .await?
            .map_or(vec![], |user| user.campaigns))
    }
}
