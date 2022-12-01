use eyre::Result;
use self::rows::{CampaignRow, PlayerRow, CampaignRows};
use sqlx::{query_as, PgPool};

mod enums;
pub mod rows;
pub mod queries;

#[derive(sqlx::FromRow)]
pub struct PlayerAndCampaigns {
    player: PlayerRow,
    campaigns: CampaignRows
}


pub async fn workaround(pool: &PgPool, player_id: i64) -> Result<PlayerAndCampaigns>{
    Ok(query_as::<_, PlayerAndCampaigns>(
        "
        SELECT
            players as player,
            ARRAY_AGG(campaigns) as campaigns
        FROM players 
        INNER JOIN campaign_players ON (players.id = campaign_players.player_id)
        INNER JOIN campaigns ON (campaigns.id = campaign_players.campaign_id)
        WHERE players.id = $1
        GROUP BY 1
        "
    ).bind(player_id).fetch_one(pool).await?)
}