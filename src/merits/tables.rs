use eyre::{eyre, Context, Result};
use sqlx::postgres::PgHasArrayType;
use std::collections::HashMap;

use crate::abilities::AbilityNameNoSubskill;
use crate::character::CharacterBuilder;
use crate::merits::{MeritTemplate, MeritType};
use crate::prerequisite::{ExaltTypePrerequisite, PrerequisiteSet};

use crate::abilities::tables::AbilityNamePostgres;
use crate::prerequisite::tables::{PrerequisiteRow, PrerequisiteTypePostgres};

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
        Self {
            name: template.name().to_owned(),
            merit_type: template.merit_type().into(),
            description: template.description().to_owned(),
            requires_detail: template.requires_detail(),
            book_title: template.data_source().book_title().map(|s| s.to_owned()),
            page_number: template.data_source().page_number(),
            creator_id: template.data_source().creator_id(),
        }
    }
}

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

impl CharacterBuilder {
    pub(crate) fn apply_merits_rows(
        mut self,
        merit_templates: Option<Vec<MeritTemplateRow>>,
        merit_details: Option<Vec<MeritDetailRow>>,
        merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
        merit_prerequisites: Option<Vec<PrerequisiteRow>>,
    ) -> Result<Self> {
        if merit_templates.is_none() {
            return Ok(self);
        }

        // Create map from merit prerequisite set id -> Vec<PrerequisiteRow>
        let set_id_to_prerequisite_rows =
            merit_prerequisites.map_or(HashMap::new(), |vec_of_rows| {
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
            let mut builder = PrerequisiteSet::create().with_id(set_id);
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
                        match row.ability_name.ok_or_else(|| {
                            eyre!("Missing ability name for ability prerequisite {}", row.id)
                        })? {
                            AbilityNamePostgres::Craft => {
                                if let Some(focus) = row.subskill_name {
                                    builder = builder.requiring_craft_focus(focus, dots);
                                } else {
                                    builder = builder
                                        .requiring_ability(AbilityNameNoSubskill::Craft, dots);
                                }
                            }
                            AbilityNamePostgres::MartialArts => {
                                if let Some(style) = row.subskill_name {
                                    builder = builder.requiring_martial_arts_style(style, dots);
                                } else {
                                    builder = builder.requiring_ability(
                                        AbilityNameNoSubskill::MartialArts,
                                        dots,
                                    );
                                }
                            }
                            other_ability_name => {
                                builder =
                                    builder.requiring_ability(other_ability_name.into(), dots);
                            }
                        }
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
                                    eyre!(
                                        "Missing ability name for attribute prerequisite {}",
                                        row.id
                                    )
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
                        builder =
                            builder.requiring_charm(row.charm_prerequisite_set_id.ok_or_else(
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
                            ExaltTypePrerequisite::DragonBlooded => {
                                builder.requiring_dragon_blooded()
                            }
                            ExaltTypePrerequisite::Spirit => builder.requiring_spirit(false),
                            ExaltTypePrerequisite::SpiritOrEclipse => {
                                builder.requiring_spirit(true)
                            }
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
                    .push(set_id_to_prerequisite_set.remove(&merit_prerequisite_set_row.id).ok_or_else(|| {
                        eyre!("Missing prerequisite set definition for set {}", &merit_prerequisite_set_row.id)
                    })?)
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
                    MeritTemplate::create_from_book(
                        merit_template_row.book_title.unwrap(),
                        merit_template_row.page_number.unwrap(),
                    )
                } else if merit_template_row.book_title.is_none()
                    && merit_template_row.page_number.is_none()
                    && merit_template_row.creator_id.is_some()
                {
                    MeritTemplate::create_custom(merit_template_row.creator_id)
                } else {
                    return Err(eyre!(
                        "Data source is inconsistent for merit template {}",
                        merit_template_row.id
                    ));
                };

                builder = builder
                    .with_id(merit_template_row.id)
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
                self = self.with_merit(
                    template,
                    row.dots
                        .try_into()
                        .wrap_err_with(|| format!("Dots {} overflow u8", row.dots))?,
                    row.detail,
                    Some(row.id),
                )?;
            }
        }

        Ok(self)
    }
}
