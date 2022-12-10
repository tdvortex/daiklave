use sqlx::{query, PgPool};
use eyre::Result;

pub async fn destroy_player(pool: &PgPool, id: i32) -> Result<()> {
    query!(
        "DELETE FROM players
        WHERE id = $1",
        id
    ).execute(pool).await?;

    Ok(())
}