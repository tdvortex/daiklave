use eyre::{Context, Result};
use sqlx::{query, PgPool};

pub async fn destroy_merits(pool: &PgPool, ids: &[i32]) -> Result<()> {
    query!(
        "DELETE FROM merits
        WHERE id IN (SELECT data.id FROM UNNEST($1::INTEGER[]) as data(id))",
        &ids: &[i32]
    )
    .execute(pool)
    .await
    .wrap_err("Database error deleting merits")?;

    Ok(())
}