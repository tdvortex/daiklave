use eyre::{eyre, Result};
use sqlx::postgres::PgHasArrayType;
use std::collections::HashMap;

use crate::abilities::AbilityNameNoSubskill;
use crate::character::{
    traits::prerequisite::{ExaltTypePrerequisite, PrerequisiteSet},
    CharacterBuilder,
};
use crate::merits::{MeritTemplate, MeritType};

use crate::abilities::tables::AbilityNamePostgres;
use crate::database::tables::prerequisites::{PrerequisiteRow, PrerequisiteTypePostgres};

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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merits")]
pub struct MeritTemplateRow {
    pub id: i32,
    pub name: String,
    pub dots: i16,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub requires_detail: bool,
}

#[derive(Debug)]
pub struct MeritTemplateInsert {
    pub name: String,
    pub dots: i16,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub requires_detail: bool,
}

impl From<MeritTemplate> for MeritTemplateInsert {
    fn from(template: MeritTemplate) -> Self {
        Self {
            name: template.name().to_owned(),
            dots: template.dots().into(),
            merit_type: template.merit_type().into(),
            description: template.description().to_owned(),
            requires_detail: template.requires_detail(),
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritPrerequisiteSetRow {
    pub id: i32,
    pub merit_id: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritDetailRow {
    pub id: i32,
    pub character_id: i32,
    pub merit_id: i32,
    pub detail: Option<String>,
}

impl CharacterBuilder {
    pub fn apply_merits_rows(
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
                                eyre!("missing dots level for ability prerequisite {}", row.id)
                            })?
                            .try_into()?;
                        match row.ability_name.ok_or_else(|| {
                            eyre!("missing ability name for ability prerequisite {}", row.id)
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
                                eyre!("missing dots level for attribute prerequisite {}", row.id)
                            })?
                            .try_into()?;
                        builder = builder.requiring_attribute(
                            row.attribute_name
                                .ok_or_else(|| {
                                    eyre!(
                                        "missing ability name for attribute prerequisite {}",
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
                                eyre!("missing dots level for essence prerequisite {}", row.id)
                            })?
                            .try_into()?;
                        builder = builder.requiring_essence_rating(dots);
                    }
                    PrerequisiteTypePostgres::Charm => {
                        builder =
                            builder.requiring_charm(row.charm_prerequisite_set_id.ok_or_else(
                                || eyre!("missing charm id for charm prerequisite {}", row.id),
                            )?);
                    }
                    PrerequisiteTypePostgres::ExaltType => {
                        let exalt_type: ExaltTypePrerequisite = row
                            .prerequisite_exalt_type
                            .ok_or_else(|| {
                                eyre!("missing exalt type for exalt type prerquisite {}", row.id)
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
            for row in rows.into_iter() {
                merit_id_to_prerequisite_sets
                    .entry(row.id)
                    .or_insert_with(Vec::new)
                    .push(set_id_to_prerequisite_set.remove(&row.id).ok_or_else(|| {
                        eyre!("missing prerequisite set definition for set {}", row.id)
                    })?)
            }
        }

        // Build a hashmap from merit id to merit template
        let mut merit_id_to_merit_template = HashMap::new();

        if let Some(template_rows) = merit_templates {
            for row in template_rows.into_iter() {
                let mut builder = MeritTemplate::create()
                    .with_id(row.id)
                    .with_name(row.name)
                    .with_description(row.description)
                    .with_merit_type(row.merit_type.into())
                    .with_dots(row.dots.try_into()?);

                if let Some(sets) = merit_id_to_prerequisite_sets.remove(&row.id) {
                    for set in sets.into_iter() {
                        builder = builder.with_prerequisite_set(set);
                    }
                }

                let template = builder.build()?;
                merit_id_to_merit_template.insert(row.id, template);
            }
        }

        if let Some(detail_rows) = merit_details {
            for row in detail_rows.into_iter() {
                let template = merit_id_to_merit_template
                    .get(&row.merit_id)
                    .ok_or_else(|| eyre!("missing template definition: {}", row.merit_id))?
                    .clone();
                self = self.with_merit(template, row.detail, Some(row.id))?;
            }
        }

        Ok(self)
    }
}
