use eyre::Result;
use super::rows::{CampaignRow, PlayerRow};
use sqlx::{query, query_as, PgPool};

impl CampaignRow {
    pub async fn get(pool: &PgPool, id: i64) -> Result<CampaignRow> {
        let campaign = query_as!(CampaignRow, "SELECT * FROM campaigns WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(campaign)
    }

    pub async fn create(
        pool: &PgPool,
        name: String,
        bot_channel: i64,
        maybe_description: Option<String>,
    ) -> Result<i64> {
        let id = query!(
            "INSERT INTO campaigns(name, bot_channel, description) VALUES ($1, $2, $3) RETURNING id",
            name,
            bot_channel,
            maybe_description
        ).fetch_one(pool).await?.id;

        Ok(id)
    }

    pub async fn remove(self, pool: &PgPool) -> Result<()> {
        query!("DELETE FROM campaigns WHERE id = $1", self.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_players_of(&self, pool: &PgPool) -> Result<Vec<PlayerRow>> {
        let players = query_as!(
            PlayerRow,
            "
            SELECT players.id, players.name 
            FROM players 
            INNER JOIN campaign_players ON (players.id = campaign_players.player_id) 
            WHERE campaign_players.campaign_id = $1
            ",
            self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(players)
    }
}

impl PlayerRow {
    pub async fn get(pool: &PgPool, id: i64) -> Result<PlayerRow> {
        let player = query_as!(PlayerRow, "SELECT * FROM players WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(player)
    }

    pub async fn create(pool: &PgPool, name: String) -> Result<i64> {
        let id = query!("INSERT INTO players(name) VALUES ($1) RETURNING id", name)
            .fetch_one(pool)
            .await?
            .id;

        Ok(id)
    }

    pub async fn remove(self, pool: &PgPool) -> Result<()> {
        query!("DELETE FROM players WHERE id = $1", self.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_campaigns_of(&self, pool: &PgPool) -> Result<Vec<CampaignRow>> {
        let campaigns = query_as!(
            CampaignRow,
            "
            SELECT campaigns.id, campaigns.name, campaigns.description, campaigns.bot_channel
            FROM campaigns 
            INNER JOIN campaign_players ON (campaigns.id = campaign_players.campaign_id) 
            WHERE campaign_players.player_id = $1
            ",
            self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(campaigns)
    }

    pub async fn get_character_stubs_of(&self, pool: &PgPool) -> Result<Vec<CharacterStub>> {
        let character_stubs = query_as!(
            CharacterStub,
            "
            SELECT characters.id as id, characters.name as name, campaigns.id as campaign_id, campaigns.name as campaign_name, players.id as player_id
            FROM players
            INNER JOIN campaign_players ON (players.id = campaign_players.player_id)
            INNER JOIN campaigns ON (campaigns.id = campaign_players.campaign_id)
            INNER JOIN characters ON (characters.campaign_player_id = campaign_players.id)
            WHERE players.id = $1
            ",
            self.id
        ).fetch_all(pool).await?;

        Ok(character_stubs)
    }
}

pub struct CharacterStub {
    pub id: i64,
    pub name: String,
    pub campaign_id: i64,
    pub campaign_name: String,
    pub player_id: i64,
}