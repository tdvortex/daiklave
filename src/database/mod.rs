use eyre::Result;
use self::rows::{PlayerRow, CampaignRows};
use sqlx::{PgPool};

mod enums;
pub mod rows;
pub mod queries;

#[derive(Debug, sqlx::FromRow)]
pub struct PlayerAndCampaigns {
    player: PlayerRow,
    campaigns: CampaignRows
}


pub async fn workaround(pool: &PgPool, player_id: i64) -> Result<PlayerAndCampaigns>{
    Ok(
        sqlx::query_as!(
            PlayerAndCampaigns,
            r#"
        SELECT
            players as "player!: PlayerRow",
            ARRAY_AGG(campaigns) as "campaigns!: CampaignRows"
        FROM players 
        INNER JOIN campaign_players ON (players.id = campaign_players.player_id)
        INNER JOIN campaigns ON (campaigns.id = campaign_players.campaign_id)
        WHERE players.id = $1
        GROUP BY 1
        "#,
        player_id
        ).fetch_one(pool).await?
    )
}