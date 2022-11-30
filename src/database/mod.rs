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
    
    pub async fn remove(pool: &PgPool, id: i64) -> Result<()> {
        query!(
            "DELETE FROM campaigns WHERE id = $1",
            id
        ).execute(pool).await?;
    
        Ok(())
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

    pub async fn remove(pool: &PgPool, id: i64) -> Result<()> {
        query!(
            "DELETE FROM players WHERE id = $1",
            id
        ).execute(pool).await?;
    
        Ok(())
    }
}