use crate::charms::MartialArtsCharm;

use super::MartialArtsStyle;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

pub(crate) async fn create_martial_arts_style_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    style: MartialArtsStyle,
    creator_id: Option<i32>,
) -> Result<i32> {
    query!(
        "INSERT INTO martial_arts_styles(name, description, book_title, page_number, creator_id)
        VALUES($1::VARCHAR(255), $2::TEXT, $3::VARCHAR(255), $4::SMALLINT, $5::INTEGER)
        RETURNING id
        ",
        style.name(),
        style.description.as_str(),
        style.data_source().book_title(),
        style.data_source().page_number(),
        creator_id
    ).fetch_one(&mut *transaction).await.wrap_err_with(|| format!("Database error attempting to insert martial arts style {}", style.name())).map(|record| record.id)
}