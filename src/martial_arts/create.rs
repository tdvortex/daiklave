use crate::charms::MartialArtsCharm;
use crate::charms::tables::CharmActionTypePostgres;
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

pub(crate) async fn create_martial_arts_charm_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    charm: MartialArtsCharm,
    creator_id: Option<i32>,
) -> Result<i32> {
    if !charm.id().is_placeholder() {
        return Ok(*charm.id());
    }

    let action_type_pg: CharmActionTypePostgres = charm.action_type().into();

    query!(
        "INSERT INTO martial_arts_charms (style_id, ability_dots_required, essence_dots_required, name, summary, description, action_type, duration, book_title, page_number, creator_id)
        VALUES ($1::INTEGER, $2::SMALLINT, $3::SMALLINT, $4::VARCHAR(255), $5::TEXT, $6::TEXT, $7::CHARMACTIONTYPE, $8::VARCHAR(255), $9::VARCHAR(255), $10::SMALLINT, $11::INTEGER)
        RETURNING id",
        *charm.style_id() as i32,
        charm.martial_arts_requirement() as i16,
        charm.essence_requirement() as i16,
        charm.name() as &str,
        charm.summary() as Option<&str>,
        charm.description() as &str,
        action_type_pg as CharmActionTypePostgres,
        charm.duration() as &str,
        charm.data_source().book_title() as Option<&str>,
        charm.data_source().page_number() as Option<i16>,
        creator_id
    ).fetch_one(&mut *transaction).await.wrap_err_with(|| format!("Database error attempting to insert martial arts charm {}", charm.name())).map(|record| record.id)
}