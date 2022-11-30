use eyre::{Result};
use sqlx::{query, query_as, PgPool};

pub struct Campaign {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub bot_channel: i64,
}

impl Campaign {
    pub async fn get(pool: &PgPool, id: i64) -> Result<Campaign> {
        let campaign = query_as!(
            Campaign,
            "SELECT * FROM campaigns WHERE id = $1",
            id
        ).fetch_one(pool).await?;
    
        Ok(campaign)
    }
    
    pub async fn create(pool: &PgPool, name: String, bot_channel: i64, maybe_description: Option<String>) -> Result<i64> {
        let id = query!(
            "INSERT INTO campaigns(name, bot_channel, description) VALUES ($1, $2, $3) RETURNING id",
            name,
            bot_channel,
            maybe_description
        ).fetch_one(pool).await?.id;
    
        Ok(id)
    }
    
    pub async fn remove(self, pool: &PgPool) -> Result<()> {
        query!(
            "DELETE FROM campaigns WHERE id = $1",
            self.id
        ).execute(pool).await?;
    
        Ok(())
    }

    pub async fn get_players_of(&self, pool: &PgPool) -> Result<Vec<Player>> {
        let players = query_as!(
            Player,
            "
            SELECT players.id, players.name 
            FROM players 
            INNER JOIN campaign_players ON (players.id = campaign_players.player_id) 
            WHERE campaign_players.campaign_id = $1
            ",
            self.id
        ).fetch_all(pool).await?;

        Ok(players)
    }
}

pub struct Player {
    pub id: i64,
    pub name: String,
}

impl Player {
    pub async fn get(pool: &PgPool, id: i64) -> Result<Player> {
        let player = query_as!(
            Player,
            "SELECT * FROM players WHERE id = $1",
            id
        ).fetch_one(pool).await?;
    
        Ok(player)
    }

    pub async fn create(pool: &PgPool, name: String) -> Result<i64> {
        let id = query!(
            "INSERT INTO players(name) VALUES ($1) RETURNING id",
            name
        ).fetch_one(pool).await?.id;
    
        Ok(id)
    }

    pub async fn remove(self, pool: &PgPool) -> Result<()> {
        query!(
            "DELETE FROM players WHERE id = $1",
            self.id
        ).execute(pool).await?;
    
        Ok(())
    }

    pub async fn get_campaigns_of(&self, pool: &PgPool) -> Result<Vec<Campaign>> {
        let campaigns = query_as!(
            Campaign,
            "
            SELECT campaigns.id, campaigns.name, campaigns.description, campaigns.bot_channel
            FROM campaigns 
            INNER JOIN campaign_players ON (campaigns.id = campaign_players.campaign_id) 
            WHERE campaign_players.player_id = $1
            ",
            self.id
        ).fetch_all(pool).await?;

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

pub enum ExaltType {
    Solar,
    Lunar,
    DragonBlooded,
}

impl TryFrom<String> for ExaltType {
    type Error = eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "SO" => Ok(Self::Solar),
            "LU" => Ok(Self::Lunar),
            "DB" => Ok(Self::DragonBlooded),
            _ => Err(eyre::eyre!("unknown exalt type encoding: {}", value))
        }
    }
}

pub struct CharacterStub {
    pub id: i64,
    pub name: String,
    pub campaign_id: i64,
    pub campaign_name: String,
    pub player_id: i64,
}

pub struct CharacterRow {
    pub id: i64,
    pub campaign_id: i64,
    pub player_id: i64,
    pub name: String,
    pub concept: Option<String>,
    pub exalt_type: String,
    pub current_willpower: i16,
    pub max_willpower: i16,
    pub current_experience: i16,
    pub total_experience: i16,
}