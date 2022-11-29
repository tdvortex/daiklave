use eyre::{Result};
use sqlx::{query, query_as, PgPool};

struct Campaign {
    id: i64,
    name: String,
    description: Option<String>,
    bot_channel: i64,
}


async fn get_campaign(pool: &PgPool, id: i64) -> Result<Campaign> {
    let campaign = query_as!(
        Campaign,
        "SELECT * FROM campaigns WHERE id = $1",
        id
    ).fetch_one(pool).await?;

    Ok(campaign)
}

async fn create_campaign(pool: &PgPool, name: String, bot_channel: i64, maybe_description: Option<String>) -> Result<i64> {
    let id = query!(
        "INSERT INTO campaigns(name, bot_channel, description) VALUES ($1, $2, $3) RETURNING id",
        name,
        bot_channel,
        maybe_description
    ).fetch_one(pool).await?.id;

    Ok(id)
}
