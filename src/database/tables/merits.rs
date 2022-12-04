use eyre::{eyre, Result};
use std::collections::HashMap;

use crate::character::{
    builder::CharacterBuilder,
    traits::{
        merits::{MeritTemplate, MeritType},
        prerequisite::Prerequisite,
    },
};

use super::prerequisites::{
    flatten_prerequite_set_hashmap_to_vec, prerequisite_row_vec_to_hashmap, PrerequisiteRow,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "MERITTYPE", rename_all = "UPPERCASE")]
pub enum MeritTypePostgres {
    Innate,
    Supernatural,
    Story,
    Purchased,
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritPrerequisiteSetRow {
    pub id: i32,
    pub merit_id: i32,
    pub prerequisite_id: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritDetailRow {
    pub id: i32,
    pub character_id: i32,
    pub merit_id: i32,
    pub detail: Option<String>,
}

type MeritHashMap = HashMap<
    i32,
    (
        HashMap<i32, Option<String>>,
        HashMap<i32, Vec<Prerequisite>>,
        MeritTemplateRow,
    ),
>;

pub fn merit_template_row_vec_to_hashmap(
    merit_template_row_vec: Vec<MeritTemplateRow>,
) -> Result<MeritHashMap> {
    merit_template_row_vec
        .into_iter()
        .map(|template_row| (template_row.id, template_row))
        .fold(Ok(HashMap::new()), |hmap_result, (id, template)| {
            hmap_result.and_then(|mut hmap| {
                if hmap.insert(id, (HashMap::new(), HashMap::new(), template)).is_some() {
                    Err(eyre!("duplicate merit template id: {}", id))
                } else {
                    Ok(hmap)
                }
            })
        })
}

pub fn insert_merit_details_into_hashmap(
    merit_template_hashmap: &mut MeritHashMap,
    merit_detail_rows_vec: Vec<MeritDetailRow>,
) -> Result<&mut MeritHashMap> {
    merit_detail_rows_vec
        .into_iter()
        .map(|merit_detail_row| {
            (
                merit_detail_row.merit_id,
                merit_detail_row.id,
                merit_detail_row.detail,
            )
        })
        .fold(
            Ok(merit_template_hashmap),
            |hmap_result, (template_id, instance_id, detail)| {
                hmap_result.and_then(|hmap| {
                    if let Some(entry) = hmap.get_mut(&template_id) {
                        if entry.0.insert(instance_id, detail).is_some() {
                            Err(eyre!("duplicate merit instance: {}", instance_id))
                        } else {
                            Ok(hmap)
                        }
                    } else {
                        Err(eyre!("merit template id {} not found", template_id))
                    }
                })
            },
        )
}

pub fn insert_prerequisite_sets_into_hashmap(
    merit_template_hashmap: &mut MeritHashMap,
    prerequisite_map: HashMap<i32, Prerequisite>,
    merit_prerequisite_set_row_vec: Vec<MeritPrerequisiteSetRow>,
) -> Result<&mut MeritHashMap> {
    merit_prerequisite_set_row_vec
        .into_iter()
        .map(|prerequisite_set_row| {
            (
                prerequisite_set_row.merit_id,
                prerequisite_set_row.id,
                prerequisite_set_row.prerequisite_id,
            )
        })
        .fold(
            Ok(merit_template_hashmap),
            |hmap_result, (template_id, set_id, prerequisite_id)| {
                hmap_result.and_then(|hmap| {
                    if let Some(entry) = hmap.get_mut(&template_id) {
                        let prerequisite = prerequisite_map
                            .get(&prerequisite_id)
                            .ok_or_else(|| eyre!("prerequisite id {} not found", prerequisite_id))?
                            .clone();
                        entry
                            .1
                            .entry(set_id)
                            .or_default()
                            .push(prerequisite);
                        Ok(hmap)
                    } else {
                        Err(eyre!("merit template id {} not found", template_id))
                    }
                })
            },
        )
}

impl CharacterBuilder {
    fn apply_merits_hashmap(&mut self, merits_hashmap: MeritHashMap) -> Result<&mut Self> {
        merits_hashmap.into_iter().fold(
            Ok(self),
            |character_result,
             (template_id, (details_hashmap, prerequisite_set_hashmap, template_row))| {
                character_result.and_then(|character| {
                    let dots = template_row.dots.try_into()?;
                    let prerequisites =
                        flatten_prerequite_set_hashmap_to_vec(prerequisite_set_hashmap);
                    let template = MeritTemplate::new(
                        template_row.name,
                        dots,
                        template_row.merit_type.into(),
                        prerequisites,
                        template_row.description,
                        template_row.requires_detail,
                        Some(template_id),
                    );

                    details_hashmap.into_iter().fold(
                        Ok(character),
                        |character_result, (instance_id, detail)| {
                            character_result.and_then(|character| {
                                character.with_merit(template.clone(), detail, Some(instance_id))
                            })
                        },
                    )
                })
            },
        )
    }
    pub fn apply_merits_rows(
        &mut self,
        merit_templates: Option<Vec<MeritTemplateRow>>,
        merit_details: Option<Vec<MeritDetailRow>>,
        merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
        merit_prerequisites: Option<Vec<PrerequisiteRow>>,
    ) -> Result<&mut Self> {
        if merit_templates.is_none() {
            return Ok(self);
        }
        let mut merits_hashmap = merit_template_row_vec_to_hashmap(merit_templates.unwrap())?;
        insert_merit_details_into_hashmap(&mut merits_hashmap, merit_details.unwrap_or_default())?;
        let prerequisite_map =
            prerequisite_row_vec_to_hashmap(merit_prerequisites.unwrap_or_default())?;
        insert_prerequisite_sets_into_hashmap(
            &mut merits_hashmap,
            prerequisite_map,
            merit_prerequisite_sets.unwrap_or_default(),
        )?;
        self.apply_merits_hashmap(merits_hashmap)
    }
}
