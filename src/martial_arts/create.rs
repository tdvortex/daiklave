use super::MartialArtsStyle;
use crate::charms::tables::{CharmActionTypePostgres, CharmCostTypePostgres, CharmKeywordPostgres};
use crate::charms::MartialArtsCharm;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

pub(crate) async fn create_martial_arts_style_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    style: &MartialArtsStyle,
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
    )
    .fetch_one(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to insert martial arts style {}",
            style.name()
        )
    })
    .map(|record| record.id)
}

pub(crate) async fn create_martial_arts_charm_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    charm: &MartialArtsCharm,
    creator_id: Option<i32>,
) -> Result<i32> {
    if !charm.id().is_placeholder() {
        return Ok(*charm.id());
    }

    let action_type_pg: CharmActionTypePostgres = charm.action_type().into();

    let charm_id = query!(
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
    ).fetch_one(&mut *transaction).await.wrap_err_with(|| format!("Database error attempting to insert martial arts charm {}", charm.name())).map(|record| record.id)?;

    let charm_keywords_pg: Vec<CharmKeywordPostgres> = charm
        .keywords()
        .iter()
        .map(|keyword| (*keyword).into())
        .collect();

    query!(
        "INSERT INTO martial_arts_charms_keywords(charm_id, keyword)
        SELECT
            $1::INTEGER as charm_id,
            data.keyword as keyword
        FROM UNNEST($2::CHARMKEYWORD[]) as data(keyword)",
        charm_id as i32,
        &charm_keywords_pg as &[CharmKeywordPostgres]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to insert keywords for martial arts charm {}",
            charm.name()
        )
    })?;

    let (cost_types_pg, cost_amounts_i16) = charm.costs().iter().fold(
        (Vec::<CharmCostTypePostgres>::new(), Vec::<i16>::new()),
        |(mut cost_types_pg, mut cost_amounts_i16), (cost_type, amount)| {
            cost_types_pg.push((*cost_type).into());
            cost_amounts_i16.push((*amount).into());
            (cost_types_pg, cost_amounts_i16)
        },
    );

    query!(
        "INSERT INTO martial_arts_charms_costs(charm_id, cost_type, amount)
        SELECT
            $1::INTEGER as charm_id,
            data.cost_type as cost_type,
            data.amount as amount
        FROM UNNEST($2::CHARMCOSTTYPE[], $3::SMALLINT[]) as data(cost_type, amount)",
        charm_id as i32,
        &cost_types_pg as &[CharmCostTypePostgres],
        &cost_amounts_i16 as &[i16]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to insert activation costs for martial arts charm {}",
            charm.name()
        )
    })?;

    Ok(charm_id)
}

pub(crate) async fn create_martial_arts_charm_tree(
    transaction: &mut Transaction<'_, Postgres>,
    child_parent_pairs: &[(i32, i32)],
) -> Result<()> {
    let (child_ids, parent_ids) = child_parent_pairs.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut child_ids, mut parent_ids), (child_id, parent_id)| {
            child_ids.push(*child_id);
            parent_ids.push(*parent_id);
            (child_ids, parent_ids)
        },
    );

    query!(
        "INSERT INTO martial_arts_charm_tree(child_id, parent_id)
        SELECT
            data.child_id as child_id,
            data.parent_id as parent_id
        FROM UNNEST($1::INTEGER[], $2::INTEGER[]) as data(child_id, parent_id)",
        &child_ids as &[i32],
        &parent_ids as &[i32],
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error attemping to insert martial arts charm prerequisites trees")?;

    Ok(())
}
