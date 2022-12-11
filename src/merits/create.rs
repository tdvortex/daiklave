use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

use crate::data_source::DataSource;
use crate::merits::tables::{MeritTemplateInsert, MeritTypePostgres};
use crate::merits::Merit;
use crate::prerequisite::create::post_prerequisites_transaction;
use crate::prerequisite::tables::{PrerequisiteInsert, PrerequisiteTypePostgres};
use crate::prerequisite::PrerequisiteType;

async fn create_merit_templates_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    merit_template_inserts: &[MeritTemplateInsert],
) -> Result<Vec<i32>> {
    let (
        names,
        merit_types,
        descriptions,
        requires_details,
        book_titles,
        page_numbers,
        creator_ids,
    ) = merit_template_inserts.iter().fold(
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        |(
            mut names,
            mut merit_types,
            mut descriptions,
            mut requires_details,
            mut book_titles,
            mut page_numbers,
            mut creator_ids,
        ),
         merit_template| {
            names.push(merit_template.name.as_str());
            merit_types.push(merit_template.merit_type);
            descriptions.push(merit_template.description.as_str());
            requires_details.push(merit_template.requires_detail);
            book_titles.push(merit_template.book_title.clone());
            page_numbers.push(merit_template.page_number);
            creator_ids.push(merit_template.creator_id);
            (
                names,
                merit_types,
                descriptions,
                requires_details,
                book_titles,
                page_numbers,
                creator_ids,
            )
        },
    );

    Ok(
        query!(
            "INSERT INTO merits(name, requires_detail, merit_type, description, book_title, page_number, creator_id)
            SELECT
                data.name,
                data.requires_detail,
                data.merit_type,
                data.description,
                data.book_title,
                data.page_number,
                data.creator_id
            FROM UNNEST($1::VARCHAR(255)[], $2::BOOLEAN[], $3::MERITTYPE[], $4::TEXT[], $5::VARCHAR(255)[], $6::SMALLINT[], $7::INTEGER[]) 
                AS data(name, requires_detail, merit_type, description, book_title, page_number, creator_id)
            RETURNING id
            ",
            &names as &[&str],
            &requires_details as &[bool],
            &merit_types as &[MeritTypePostgres],
            &descriptions as &[&str],
            &book_titles as &[Option<String>],
            &page_numbers as &[Option<i16>],
            &creator_ids as &[Option<i32>]
        ).fetch_all(&mut *transaction).await.wrap_err("Database error trying to create new merit templates")?.into_iter().map(|record| record.id).collect()
    )
}

async fn create_merit_prerequisite_sets_transaction(
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
    .await
    .wrap_err("Database error trying to create new merit prerequisite sets")?
    .into_iter()
    .map(|record| record.id)
    .collect())
}

pub(crate) async fn post_merits_details_transaction(
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
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to assign merits to character {}",
            character_id
        )
    })?
    .into_iter()
    .map(|record| record.id)
    .collect())
}

pub(crate) async fn create_new_merits_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    new_merits: Vec<Merit>,
    character_id: i32,
) -> Result<()> {
    // Create base merit templates and get their ids
    let mut merit_template_inserts = Vec::new();
    for merit in new_merits.iter() {
        let creator_id = if let DataSource::Book(_) = merit.data_source() {
            None
        } else {
            Some(character_id)
        };

        merit_template_inserts.push(MeritTemplateInsert {
            name: merit.name().to_owned(),
            merit_type: merit.merit_type().into(),
            description: merit.description().into(),
            requires_detail: merit.requires_detail(),
            book_title: merit.data_source().book_title().map(|s| s.to_owned()),
            page_number: merit.data_source().page_number(),
            creator_id,
        });
    }
    let new_template_ids = create_merit_templates_transaction(transaction, &merit_template_inserts)
        .await
        .wrap_err("Error creating new merit templates")?;

    // Create prerequisite sets for all newly created templates that have them
    let mut merit_template_ids_repeated = Vec::new();
    for (merit, merit_id) in new_merits.iter().zip(new_template_ids.iter()) {
        (0..merit.prerequisites().len()).for_each(|_| merit_template_ids_repeated.push(*merit_id));
    }
    let new_set_ids =
        create_merit_prerequisite_sets_transaction(transaction, &merit_template_ids_repeated)
            .await
            .wrap_err("Error linking prerequisite sets to merits")?;

    // Create the prerequisites in those sets and link them
    let mut prerequisites = Vec::new();
    for merit in new_merits.iter() {
        for (set, set_id) in merit.prerequisites().iter().zip(new_set_ids.iter()) {
            let merit_prerequisite_set_id = Some(*set_id);
            let charm_prerequisite_set_id = None;

            for prerequisite in set.iter() {
                prerequisites.push(match prerequisite.prerequisite_type() {
                    PrerequisiteType::Ability(ability_prerequisite) => PrerequisiteInsert {
                        merit_prerequisite_set_id,
                        charm_prerequisite_set_id,
                        prerequisite_type: PrerequisiteTypePostgres::Ability,
                        ability_name: Some(ability_prerequisite.ability_name.into()),
                        subskill_name: ability_prerequisite
                            .subskill
                            .as_deref()
                            .map(|s| s.to_owned()),
                        attribute_name: None,
                        dots: Some(ability_prerequisite.dots.into()),
                        charm_id: None,
                        exalt_type: None,
                    },
                    PrerequisiteType::Attribute(attribute_prerequisite) => PrerequisiteInsert {
                        merit_prerequisite_set_id,
                        charm_prerequisite_set_id,
                        prerequisite_type: PrerequisiteTypePostgres::Attribute,
                        ability_name: None,
                        subskill_name: None,
                        attribute_name: Some(attribute_prerequisite.attribute_name.into()),
                        dots: Some(attribute_prerequisite.dots.into()),
                        charm_id: None,
                        exalt_type: None,
                    },
                    PrerequisiteType::Essence(dots) => PrerequisiteInsert {
                        merit_prerequisite_set_id,
                        charm_prerequisite_set_id,
                        prerequisite_type: PrerequisiteTypePostgres::Essence,
                        ability_name: None,
                        subskill_name: None,
                        attribute_name: None,
                        dots: Some((*dots).into()),
                        charm_id: None,
                        exalt_type: None,
                    },
                    PrerequisiteType::Charm(charm_id) => PrerequisiteInsert {
                        merit_prerequisite_set_id,
                        charm_prerequisite_set_id,
                        prerequisite_type: PrerequisiteTypePostgres::Charm,
                        ability_name: None,
                        subskill_name: None,
                        attribute_name: None,
                        dots: None,
                        charm_id: Some(*charm_id),
                        exalt_type: None,
                    },
                    PrerequisiteType::ExaltType(exalt_type) => PrerequisiteInsert {
                        merit_prerequisite_set_id,
                        charm_prerequisite_set_id,
                        prerequisite_type: PrerequisiteTypePostgres::ExaltType,
                        ability_name: None,
                        subskill_name: None,
                        attribute_name: None,
                        dots: None,
                        charm_id: None,
                        exalt_type: Some((*exalt_type).into()),
                    },
                });
            }
        }
    }
    post_prerequisites_transaction(transaction, &prerequisites)
        .await
        .wrap_err("Error attempting to create prerequisites")?;

    // Link those new merits to the character
    let mut merit_details = Vec::new();
    for (merit, merit_id) in new_merits.iter().zip(new_template_ids.iter()) {
        merit_details.push((*merit_id, merit.detail().map(|s| s.to_owned())));
    }
    post_merits_details_transaction(transaction, merit_details, character_id)
        .await
        .wrap_err_with(|| {
            format!(
                "Error attempting to specify merit details for character {}",
                character_id
            )
        })?;

    Ok(())
}
