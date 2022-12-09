use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::database::tables::merits::{MeritTemplateInsert, MeritTypePostgres};

async fn post_merit_templates_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    merit_templates: &[MeritTemplateInsert],
) -> Result<Vec<i32>> {
    let (names, dots_vec, merit_types, descriptions, requires_details) =
        merit_templates.iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |(mut names, mut dots_vec, mut merit_types, mut descriptions, mut requires_details),
             merit_template| {
                names.push(merit_template.name.as_str());
                dots_vec.push(merit_template.dots);
                merit_types.push(merit_template.merit_type);
                descriptions.push(merit_template.description.as_str());
                requires_details.push(merit_template.requires_detail);
                (names, dots_vec, merit_types, descriptions, requires_details)
            },
        );

    Ok(
        query!(
            "INSERT INTO merits(name, requires_detail, dots, merit_type, description)
            SELECT
                data.name,
                data.requires_detail,
                data.dots,
                data.merit_type,
                data.description
            FROM UNNEST($1::VARCHAR(255)[], $2::BOOLEAN[], $3::SMALLINT[], $4::MERITTYPE[], $5::TEXT[]) as data(name, requires_detail, dots, merit_type, description)
            RETURNING id
            ",
            &names as &[&str],
            &requires_details as &[bool],
            &dots_vec as &[i16],
            &merit_types as &[MeritTypePostgres],
            &descriptions as &[&str]
        ).fetch_all(&mut *transaction).await?.into_iter().map(|record| record.id).collect()
    )
}

async fn post_merit_prerequisite_sets_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    merit_template_ids_repeated: &[i32],
) -> Result<Vec<i32>> {
    Ok(query!(
        "INSERT INTO merit_prerequisite_sets(merit_id)
        SELECT data.merit_id FROM UNNEST($1::INTEGER[]) AS data(merit_id)
        RETURNING id",
        merit_template_ids_repeated
    )
    .fetch_all(&mut *transaction)
    .await?
    .into_iter()
    .map(|record| record.id)
    .collect())
}

async fn post_merits_details_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    merit_details: Vec<(i32, Option<String>)>,
    character_id: i32,
) -> Result<Vec<i32>> {
    let (merit_template_ids, details) = merit_details.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut ids, mut details), (id, detail)| {
            ids.push(id);
            details.push(detail);
            (ids, details)
        },
    );

    Ok(query!(
        "INSERT INTO character_merits(character_id, merit_id, detail)
        SELECT
            $1::INTEGER,
            data.merit_id,
            data.detail
        FROM UNNEST($2::INTEGER[], $3::VARCHAR(255)[]) as data(merit_id, detail)
        RETURNING id
        ",
        character_id as i32,
        &merit_template_ids as &[i32],
        &details as &[Option<String>]
    )
    .fetch_all(&mut *transaction)
    .await?
    .into_iter()
    .map(|record| record.id)
    .collect())
}
