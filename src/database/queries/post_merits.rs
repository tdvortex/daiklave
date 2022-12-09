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
