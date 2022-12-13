use eyre::{Context, Result};
use sqlx::{query, PgPool};

pub async fn destroy_character(pool: &PgPool, id: i32) -> Result<()> {
    query!(
        "DELETE FROM characters
        WHERE id = $1",
        id as i32
    )
    .execute(pool)
    .await
    .wrap_err_with(|| format!("Database error deleting character {}", id))?;

    Ok(())
}
