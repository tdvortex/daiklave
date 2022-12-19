use std::collections::HashMap;

use daiklave_core::{
    character::CharacterBuilder,
    data_source::DataSource,
    id::Id,
    merits::{Merit, MeritDiff, MeritTemplate, MeritType},
    prerequisite::{ExaltTypePrerequisite, PrerequisiteSet, PrerequisiteType},
};
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

use eyre::{eyre, Result, WrapErr};

use crate::{abilities::AbilityNameVanillaPostgres, attributes::AttributeNamePostgres};

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritPrerequisiteSetRow {
    pub id: i32,
    pub merit_id: i32,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "character_merits")]
pub struct MeritDetailRow {
    pub id: i32,
    pub character_id: i32,
    pub merit_id: i32,
    pub dots: i16,
    pub detail: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "PREREQUISITETYPE", rename_all = "UPPERCASE")]
pub enum PrerequisiteTypePostgres {
    Ability,
    Attribute,
    Essence,
    Charm,
    ExaltType,
}

impl PgHasArrayType for PrerequisiteTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_PREREQUISITETYPE")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "PREREQUISITEEXALTTYPE", rename_all = "UPPERCASE")]
pub enum PrerequisiteExaltTypePostgres {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

impl PgHasArrayType for PrerequisiteExaltTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_PREREQUISITEEXALTTYPE")
    }
}

impl From<PrerequisiteExaltTypePostgres> for ExaltTypePrerequisite {
    fn from(exalt_type: PrerequisiteExaltTypePostgres) -> Self {
        match exalt_type {
            PrerequisiteExaltTypePostgres::Solar => Self::Solar,
            PrerequisiteExaltTypePostgres::Lunar => Self::Lunar,
            PrerequisiteExaltTypePostgres::DragonBlooded => Self::DragonBlooded,
            PrerequisiteExaltTypePostgres::Spirit => Self::Spirit,
            PrerequisiteExaltTypePostgres::SpiritOrEclipse => Self::SpiritOrEclipse,
        }
    }
}

impl From<ExaltTypePrerequisite> for PrerequisiteExaltTypePostgres {
    fn from(exalt_type: ExaltTypePrerequisite) -> Self {
        match exalt_type {
            ExaltTypePrerequisite::Solar => Self::Solar,
            ExaltTypePrerequisite::Lunar => Self::Lunar,
            ExaltTypePrerequisite::DragonBlooded => Self::DragonBlooded,
            ExaltTypePrerequisite::Spirit => Self::Spirit,
            ExaltTypePrerequisite::SpiritOrEclipse => Self::SpiritOrEclipse,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrerequisiteRow {
    pub id: i32,
    pub merit_prerequisite_set_id: Option<i32>,
    pub charm_prerequisite_set_id: Option<i32>,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNameVanillaPostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub prerequisite_charm_id: Option<i32>,
    pub prerequisite_exalt_type: Option<PrerequisiteExaltTypePostgres>,
}

impl sqlx::Type<sqlx::Postgres> for PrerequisiteRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("prerequisites")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for PrerequisiteRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let merit_prerequisite_set_id = decoder.try_decode::<Option<i32>>()?;
        let charm_prerequisite_set_id = decoder.try_decode::<Option<i32>>()?;
        let prerequisite_type = decoder.try_decode::<PrerequisiteTypePostgres>()?;
        let ability_name = decoder.try_decode::<Option<AbilityNameVanillaPostgres>>()?;
        let subskill_name = decoder.try_decode::<Option<String>>()?;
        let attribute_name = decoder.try_decode::<Option<AttributeNamePostgres>>()?;
        let dots = decoder.try_decode::<Option<i16>>()?;
        let prerequisite_charm_id = decoder.try_decode::<Option<i32>>()?;
        let prerequisite_exalt_type =
            decoder.try_decode::<Option<PrerequisiteExaltTypePostgres>>()?;

        Ok(Self {
            id,
            merit_prerequisite_set_id,
            charm_prerequisite_set_id,
            prerequisite_type,
            ability_name,
            subskill_name,
            attribute_name,
            dots,
            prerequisite_charm_id,
            prerequisite_exalt_type,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "MERITTYPE", rename_all = "UPPERCASE")]
pub enum MeritTypePostgres {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

impl PgHasArrayType for MeritTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_MERITTYPE")
    }
}

impl From<MeritTypePostgres> for MeritType {
    fn from(merit_type_postgres: MeritTypePostgres) -> Self {
        match merit_type_postgres {
            MeritTypePostgres::Innate => Self::Innate,
            MeritTypePostgres::Supernatural => Self::Supernatural,
            MeritTypePostgres::Story => Self::Story,
            MeritTypePostgres::Purchased => Self::Purchased,
        }
    }
}

impl From<MeritType> for MeritTypePostgres {
    fn from(merit_type: MeritType) -> Self {
        match merit_type {
            MeritType::Innate => Self::Innate,
            MeritType::Supernatural => Self::Supernatural,
            MeritType::Story => Self::Story,
            MeritType::Purchased => Self::Purchased,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeritTemplateRow {
    pub id: i32,
    pub name: String,
    pub requires_detail: bool,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for MeritTemplateRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("merits")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MeritTemplateRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let requires_detail = decoder.try_decode::<bool>()?;
        let merit_type = decoder.try_decode::<MeritTypePostgres>()?;
        let description = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            requires_detail,
            merit_type,
            description,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct MeritTemplateInsert {
    pub name: String,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub requires_detail: bool,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
    pub creator_id: Option<i32>,
}

impl From<MeritTemplate> for MeritTemplateInsert {
    fn from(template: MeritTemplate) -> Self {
        let nulled_creator_id = template.data_source().creator_id().and_then(|id| {
            if id.is_placeholder() {
                None
            } else {
                Some(*id)
            }
        });
        Self {
            name: template.name().to_owned(),
            merit_type: template.merit_type().into(),
            description: template.description().to_owned(),
            requires_detail: template.requires_detail(),
            book_title: template.data_source().book_title().map(|s| s.to_owned()),
            page_number: template.data_source().page_number(),
            creator_id: nulled_creator_id,
        }
    }
}

pub fn apply_merits_rows(
    mut builder: CharacterBuilder,
    merit_templates: Option<Vec<MeritTemplateRow>>,
    merit_details: Option<Vec<MeritDetailRow>>,
    merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    merit_prerequisites: Option<Vec<PrerequisiteRow>>,
) -> Result<CharacterBuilder> {
    if merit_templates.is_none() {
        return Ok(builder);
    }

    // Create map from merit prerequisite set id -> Vec<PrerequisiteRow>
    let set_id_to_prerequisite_rows = merit_prerequisites.map_or(HashMap::new(), |vec_of_rows| {
        vec_of_rows
            .into_iter()
            .filter_map(|row| {
                row.merit_prerequisite_set_id
                    .map(|merit_prerequisite_set_id| (merit_prerequisite_set_id, row))
            })
            .fold(
                HashMap::new(),
                |mut hashmap, (merit_prerequisite_set_id, row)| {
                    hashmap
                        .entry(merit_prerequisite_set_id)
                        .or_insert_with(Vec::new)
                        .push(row);
                    hashmap
                },
            )
    });

    // Compile each Vec<PrerequisiteRow> into PrerequisiteSet using builder
    let mut set_id_to_prerequisite_set = HashMap::new();

    for (set_id, vec_of_rows) in set_id_to_prerequisite_rows.into_iter() {
        let mut builder = PrerequisiteSet::create().with_database_id(set_id);
        for row in vec_of_rows.into_iter() {
            match row.prerequisite_type {
                PrerequisiteTypePostgres::Ability => {
                    let dots = row
                        .dots
                        .ok_or_else(|| {
                            eyre!("Missing dots level for ability prerequisite {}", row.id)
                        })?
                        .try_into()
                        .wrap_err("Ability prerequisite dots overflow u8")?;

                    let ability_name = row.ability_name.ok_or_else(|| {
                        eyre!("Missing ability name for ability prerequisite {}", row.id)
                    })?;
                    builder = builder.requiring_ability(ability_name.into(), dots);
                }
                PrerequisiteTypePostgres::Attribute => {
                    let dots = row
                        .dots
                        .ok_or_else(|| {
                            eyre!("Missing dots level for attribute prerequisite {}", row.id)
                        })?
                        .try_into()
                        .wrap_err("Attribute prerequisite dots overflow u8")?;
                    builder = builder.requiring_attribute(
                        row.attribute_name
                            .ok_or_else(|| {
                                eyre!("Missing ability name for attribute prerequisite {}", row.id)
                            })?
                            .into(),
                        dots,
                    );
                }
                PrerequisiteTypePostgres::Essence => {
                    let dots = row
                        .dots
                        .ok_or_else(|| {
                            eyre!("Missing dots level for essence prerequisite {}", row.id)
                        })?
                        .try_into()
                        .wrap_err("Essence prerequisite dots overflow u8")?;
                    builder = builder.requiring_essence_rating(dots);
                }
                PrerequisiteTypePostgres::Charm => {
                    builder = builder.requiring_charm(row.charm_prerequisite_set_id.ok_or_else(
                        || eyre!("Missing charm id for charm prerequisite {}", row.id),
                    )?);
                }
                PrerequisiteTypePostgres::ExaltType => {
                    let exalt_type: ExaltTypePrerequisite = row
                        .prerequisite_exalt_type
                        .ok_or_else(|| {
                            eyre!("Missing exalt type for exalt type prerquisite {}", row.id)
                        })?
                        .into();
                    builder = match exalt_type {
                        ExaltTypePrerequisite::Solar => builder.requiring_solar(),
                        ExaltTypePrerequisite::Lunar => builder.requiring_lunar(),
                        ExaltTypePrerequisite::DragonBlooded => builder.requiring_dragon_blooded(),
                        ExaltTypePrerequisite::Spirit => builder.requiring_spirit(false),
                        ExaltTypePrerequisite::SpiritOrEclipse => builder.requiring_spirit(true),
                    }
                }
            }
        }

        set_id_to_prerequisite_set.insert(set_id, builder.build());
    }

    // Build a hashmap from merit id to Vec<PrerequisiteSet>
    let mut merit_id_to_prerequisite_sets = HashMap::new();

    if let Some(rows) = merit_prerequisite_sets {
        for merit_prerequisite_set_row in rows.into_iter() {
            merit_id_to_prerequisite_sets
                .entry(merit_prerequisite_set_row.merit_id)
                .or_insert_with(Vec::new)
                .push(
                    set_id_to_prerequisite_set
                        .remove(&merit_prerequisite_set_row.id)
                        .ok_or_else(|| {
                            eyre!(
                                "Missing prerequisite set definition for set {}",
                                &merit_prerequisite_set_row.id
                            )
                        })?,
                )
        }
    }
    // Build a hashmap from merit id to merit template
    let mut merit_id_to_merit_template = HashMap::new();

    if let Some(template_rows) = merit_templates {
        for merit_template_row in template_rows.into_iter() {
            let mut builder = if merit_template_row.book_title.is_some()
                && merit_template_row.page_number.is_some()
                && merit_template_row.creator_id.is_none()
            {
                MeritTemplate::from_book(
                    Id::Database(merit_template_row.id),
                    merit_template_row.book_title.unwrap(),
                    merit_template_row.page_number.unwrap(),
                )
            } else if merit_template_row.book_title.is_none()
                && merit_template_row.page_number.is_none()
                && merit_template_row.creator_id.is_some()
            {
                MeritTemplate::custom(
                    Id::Database(merit_template_row.id),
                    Id::Database(merit_template_row.creator_id.unwrap()),
                )
            } else {
                return Err(eyre!(
                    "Data source is inconsistent for merit template {}",
                    merit_template_row.id
                ));
            };

            builder = builder
                .with_database_id(merit_template_row.id)
                .with_name(merit_template_row.name)
                .with_description(merit_template_row.description)
                .with_merit_type(merit_template_row.merit_type.into());

            builder = if merit_template_row.requires_detail {
                builder.requiring_detail()
            } else {
                builder.not_requiring_detail()
            };

            if let Some(sets) = merit_id_to_prerequisite_sets.remove(&merit_template_row.id) {
                for set in sets.into_iter() {
                    builder = builder.with_prerequisite_set(set);
                }
            }

            let template = builder.build().wrap_err_with(|| {
                format!(
                    "Error attempting to build merit template {} from rows",
                    merit_template_row.id
                )
            })?;
            merit_id_to_merit_template.insert(merit_template_row.id, template);
        }
    }

    if let Some(detail_rows) = merit_details {
        for row in detail_rows.into_iter() {
            let template = merit_id_to_merit_template
                .get(&row.merit_id)
                .ok_or_else(|| eyre!("missing template definition: {}", row.merit_id))?
                .clone();
            builder = builder.with_merit_ignore_prerequisites(
                template,
                row.dots
                    .try_into()
                    .wrap_err_with(|| format!("Dots {} overflow u8", row.dots))?,
                row.detail,
                Id::Database(row.id),
            )?;
        }
    }

    Ok(builder)
}

pub async fn update_merits(
    merits_diff: MeritDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if !merits_diff.remove_merit_instances.is_empty() {
        query!(
            "DELETE FROM character_merits
            WHERE character_id = $1::INTEGER AND id IN (SELECT data.id FROM UNNEST($2::INTEGER[]) as data(id))",
            character_id,
            &merits_diff.remove_merit_instances as &[i32]
        ).execute(&mut *transaction).await.wrap_err("Database error attempting to delete character merits")?;
    }

    create_new_merits_transaction(
        transaction,
        merits_diff.insert_merit_templates,
        character_id,
    )
    .await
    .wrap_err("Error attempting to create new merits")?;

    post_merits_details_transaction(transaction, merits_diff.insert_merit_instance, character_id)
        .await
        .wrap_err_with(|| {
            format!(
                "Error attempting to assign merits to character {}",
                character_id
            )
        })?;
    Ok(())
}

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
    merit_details: Vec<(i32, u8, Option<String>)>,
    character_id: i32,
) -> Result<Vec<i32>> {
    let (merit_template_ids, dots_vec, details) = merit_details.into_iter().fold(
        (Vec::new(), Vec::<i16>::new(), Vec::new()),
        |(mut ids, mut dots_vec, mut details), (id, dots, detail)| {
            ids.push(id);
            dots_vec.push(dots.into());
            details.push(detail);
            (ids, dots_vec, details)
        },
    );

    Ok(query!(
        "INSERT INTO character_merits(character_id, merit_id, dots, detail)
        SELECT
            $1::INTEGER,
            data.merit_id,
            data.dots,
            data.detail
        FROM UNNEST($2::INTEGER[], $3::SMALLINT[], $4::VARCHAR(255)[]) as data(merit_id, dots, detail)
        RETURNING id
        ",
        character_id as i32,
        &merit_template_ids as &[i32],
        &dots_vec as &[i16],
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

#[derive(Debug)]
pub struct PrerequisiteInsert {
    pub merit_prerequisite_set_id: Option<i32>,
    pub charm_prerequisite_set_id: Option<i32>,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNameVanillaPostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub charm_id: Option<i32>,
    pub exalt_type: Option<PrerequisiteExaltTypePostgres>,
}

pub(crate) async fn post_prerequisites_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    prerequisites: &[PrerequisiteInsert],
) -> Result<Vec<i32>> {
    let (
        merit_prerequisite_ids,
        charm_prerequisite_ids,
        prerequisite_types,
        ability_names,
        subskill_names,
        attribute_names,
        dots_vec,
        prerequisite_charm_ids,
        prerequisite_exalt_types,
    ) = prerequisites.iter().fold(
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        |(
            mut merit_prerequisite_ids,
            mut charm_prerequisite_ids,
            mut prerequisite_types,
            mut ability_names,
            mut subskill_names,
            mut attribute_names,
            mut dots_vec,
            mut prerequisite_charm_ids,
            mut prerequisite_exalt_types,
        ),
         prerequisite_insert| {
            merit_prerequisite_ids.push(prerequisite_insert.merit_prerequisite_set_id);
            charm_prerequisite_ids.push(prerequisite_insert.charm_prerequisite_set_id);
            prerequisite_types.push(prerequisite_insert.prerequisite_type);
            ability_names.push(prerequisite_insert.ability_name);
            subskill_names.push(prerequisite_insert.subskill_name.as_deref());
            attribute_names.push(prerequisite_insert.attribute_name);
            dots_vec.push(prerequisite_insert.dots);
            prerequisite_charm_ids.push(prerequisite_insert.charm_id);
            prerequisite_exalt_types.push(prerequisite_insert.exalt_type);
            (
                merit_prerequisite_ids,
                charm_prerequisite_ids,
                prerequisite_types,
                ability_names,
                subskill_names,
                attribute_names,
                dots_vec,
                prerequisite_charm_ids,
                prerequisite_exalt_types,
            )
        },
    );

    Ok(query!(
        "INSERT INTO prerequisites(merit_prerequisite_set_id, charm_prerequisite_set_id, prerequisite_type, ability_name, subskill_name, attribute_name, dots, charm_id, prerequisite_exalt_type)
        SELECT
            data.merit_prerequisite_set_id, 
            data.charm_prerequisite_set_id, 
            data.prerequisite_type,
            data.ability_name,
            data.subskill_name,
            data.attribute_name,
            data.dots,
            data.charm_id,
            data.prerequisite_exalt_type
        FROM UNNEST($1::INTEGER[], $2::INTEGER[], $3::PREREQUISITETYPE[], $4::ABILITYNAMEVANILLA[], $5::VARCHAR(255)[], $6::ATTRIBUTENAME[], $7::SMALLINT[], $8::INTEGER[], $9::PREREQUISITEEXALTTYPE[])
            AS data(merit_prerequisite_set_id, charm_prerequisite_set_id, prerequisite_type, ability_name, subskill_name, attribute_name, dots, charm_id, prerequisite_exalt_type)
        RETURNING id",
        &merit_prerequisite_ids as &[Option<i32>],
        &charm_prerequisite_ids as &[Option<i32>],
        &prerequisite_types as &[PrerequisiteTypePostgres],
        &ability_names as &[Option<AbilityNameVanillaPostgres>],
        &subskill_names as &[Option<&str>],
        &attribute_names as &[Option<AttributeNamePostgres>],
        &dots_vec as &[Option<i16>],
        &prerequisite_charm_ids as &[Option<i32>],
        &prerequisite_exalt_types as &[Option<PrerequisiteExaltTypePostgres>]
    ).fetch_all(&mut *transaction).await.wrap_err("Database error attempting to create new prerequisites")?.into_iter().map(|record| record.id).collect())
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
    let prerequisites = new_merits
        .iter()
        .flat_map(|new_merit| new_merit.prerequisites())
        .zip(new_set_ids.iter())
        .fold(
            Vec::new(),
            |mut prerequisites, (prerequisite_set, set_id)| {
                let merit_prerequisite_set_id = Some(*set_id);
                let charm_prerequisite_set_id = None;

                for prerequisite in prerequisite_set.iter() {
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
                prerequisites
            },
        );

    post_prerequisites_transaction(transaction, &prerequisites)
        .await
        .wrap_err("Error attempting to create prerequisites")?;

    // Link those new merits to the character
    let mut merit_details = Vec::new();
    for (merit, merit_id) in new_merits.iter().zip(new_template_ids.iter()) {
        merit_details.push((
            *merit_id,
            merit.dots(),
            merit.detail().map(|s| s.to_owned()),
        ));
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
