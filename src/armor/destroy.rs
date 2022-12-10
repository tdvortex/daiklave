use sqlx::{query, PgPool};
use eyre::{Result, Context};

pub async fn destroy_armor(pool: &PgPool, ids: Vec<i32>) -> Result<()> {
    query!(
        "DELETE FROM armor
        WHERE id IN (SELECT data.id FROM UNNEST($1::INTEGER[]) as data(id))",
        &ids: &[i32]
    ).execute(pool).await.wrap_err("Database error deleting armor items")?;

    Ok(())
}