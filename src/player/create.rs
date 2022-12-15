use eyre::Result;
use sqlx::{query, PgPool};

use super::Player;

pub async fn create_player(pool: &PgPool, name: String) -> Result<Player> {
    let id = query!(
        "INSERT INTO players(name)
        VALUES ($1::VARCHAR(255))
        RETURNING id
        ",
        &name.as_str() as &str
    )
    .fetch_one(pool)
    .await?
    .id;

    Ok(Player {
        database_id: id,
        name,
    })
}
