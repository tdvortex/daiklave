use std::collections::HashMap;

use crate::character::traits::{
    abilities::AbilityNameNoSubskill,
    prerequisite::{
        AbilityPrerequisite, AttributePrerequisite, ExaltTypePrerequisite, Prerequisite,
        PrerequisiteSet, PrerequisiteType,
    },
};
use eyre::{eyre, Report, Result};
use sqlx::postgres::PgHasArrayType;

use super::{abilities::AbilityNamePostgres, attributes::AttributeNamePostgres};

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

#[derive(Debug)]
pub struct PrerequisiteRow {
    pub id: i32,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNamePostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub prerequisite_charm_id: Option<i32>,
    pub prerequisite_exalt_type: Option<PrerequisiteExaltTypePostgres>,
}

#[derive(Debug)]
pub struct PrerequisiteInsert {
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNamePostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub charm_id: Option<i32>,
    pub exalt_type: Option<PrerequisiteExaltTypePostgres>,
}

impl From<Prerequisite> for PrerequisiteInsert {
    fn from(prerequisite: Prerequisite) -> Self {
        match prerequisite.prerequisite_type() {
            PrerequisiteType::Ability(ability_prerequisite) => Self {
                    prerequisite_type: PrerequisiteTypePostgres::Ability,
                    ability_name: Some(ability_prerequisite.ability_name.into()),
                    subskill_name: ability_prerequisite.subskill.clone(),
                    attribute_name: None,
                    dots: Some(ability_prerequisite.dots.into()),
                    charm_id: None,
                    exalt_type: None,
                },
            PrerequisiteType::Attribute(attribute_prerequisite) => Self {
                    prerequisite_type: PrerequisiteTypePostgres::Attribute,
                    ability_name: None,
                    subskill_name: None,
                    attribute_name: Some(attribute_prerequisite.attribute_name.into()),
                    dots: Some(attribute_prerequisite.dots.into()),
                    charm_id: None,
                    exalt_type: None,
                },
            PrerequisiteType::Essence(dots) => Self {
                prerequisite_type: PrerequisiteTypePostgres::Essence,
                ability_name: None,
                subskill_name: None,
                attribute_name: None,
                dots: Some((*dots).into()),
                charm_id: None,
                exalt_type: None,
            },
            PrerequisiteType::Charm(charm_id) => Self {
                prerequisite_type: PrerequisiteTypePostgres::Charm,
                ability_name: None,
                subskill_name: None,
                attribute_name: None,
                dots: None,
                charm_id: Some(*charm_id),
                exalt_type: None,
            },
            PrerequisiteType::ExaltType(exalt_type_prerequisite) => Self {
                prerequisite_type: PrerequisiteTypePostgres::ExaltType,
                ability_name: None,
                subskill_name: None,
                attribute_name: None,
                dots: None,
                charm_id: None,
                exalt_type: Some((*exalt_type_prerequisite).into()),
            },
        }
    }
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
        let prerequisite_type = decoder.try_decode::<PrerequisiteTypePostgres>()?;
        let ability_name = decoder.try_decode::<Option<AbilityNamePostgres>>()?;
        let subskill_name = decoder.try_decode::<Option<String>>()?;
        let attribute_name = decoder.try_decode::<Option<AttributeNamePostgres>>()?;
        let dots = decoder.try_decode::<Option<i16>>()?;
        let prerequisite_charm_id = decoder.try_decode::<Option<i32>>()?;
        let prerequisite_exalt_type =
            decoder.try_decode::<Option<PrerequisiteExaltTypePostgres>>()?;

        Ok(Self {
            id,
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

impl TryInto<Prerequisite> for PrerequisiteRow {
    type Error = Report;

    fn try_into(self) -> Result<Prerequisite, Self::Error> {
        match self.prerequisite_type {
            PrerequisiteTypePostgres::Ability => {
                if self.ability_name.is_none() {
                    return Err(eyre!(
                        "ability name must be specified for ability prerequisite"
                    ));
                }

                if self.dots.is_none() {
                    return Err(eyre!(
                        "dots level must be specified for ability prerequisite"
                    ));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Ability(AbilityPrerequisite {
                        ability_name: self.ability_name.unwrap().into(),
                        subskill: self.subskill_name,
                        dots: self.dots.unwrap().try_into()?,
                    }),
                    Some(self.id),
                ))
            }
            PrerequisiteTypePostgres::Attribute => {
                if self.attribute_name.is_none() {
                    return Err(eyre!(
                        "attribute name must be specified for attribute prerequisite"
                    ));
                }

                if self.dots.is_none() {
                    return Err(eyre!(
                        "dots level must be specified for attribute prerequisite"
                    ));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Attribute(AttributePrerequisite {
                        attribute_name: self.attribute_name.unwrap().into(),
                        dots: self.dots.unwrap().try_into()?,
                    }),
                    Some(self.id),
                ))
            }
            PrerequisiteTypePostgres::Essence => {
                if self.dots.is_none() {
                    return Err(eyre!(
                        "dots level must be specified for essence prerequisite"
                    ));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Essence(self.dots.unwrap().try_into()?),
                    Some(self.id),
                ))
            }
            PrerequisiteTypePostgres::Charm => {
                if self.prerequisite_charm_id.is_none() {
                    return Err(eyre!("charm id must be specified for charm prerequisite"));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::Charm(self.prerequisite_charm_id.unwrap()),
                    Some(self.id),
                ))
            }
            PrerequisiteTypePostgres::ExaltType => {
                if self.prerequisite_exalt_type.is_none() {
                    return Err(eyre!(
                        "exalt type must be specified for exalt type prerequisite"
                    ));
                }

                Ok(Prerequisite::new(
                    PrerequisiteType::ExaltType(self.prerequisite_exalt_type.unwrap().into()),
                    Some(self.id),
                ))
            }
        }
    }
}

pub fn prerequisite_row_vec_to_hashmap(
    prerequisite_row_vec: Vec<PrerequisiteRow>,
) -> Result<HashMap<i32, Prerequisite>> {
    prerequisite_row_vec
        .into_iter()
        .map(|prerequisite_row| (prerequisite_row.id, prerequisite_row.try_into()))
        .fold(Ok(HashMap::new()), |hmap_result, (id, prereq_result)| {
            let prereq = prereq_result?;
            hmap_result.and_then(|mut hmap| {
                if hmap.insert(id, prereq).is_some() {
                    Err(eyre!("duplicate prerequisite id: {}", id))
                } else {
                    Ok(hmap)
                }
            })
        })
}

pub fn flatten_prerequite_set_hashmap_to_vec(
    prerequisite_set_hashmap: HashMap<i32, Vec<Prerequisite>>,
) -> Vec<PrerequisiteSet> {
    prerequisite_set_hashmap
        .into_iter()
        .map(|(id, prerequisites)| {
            prerequisites
                .into_iter()
                .fold(
                    PrerequisiteSet::create().with_id(id),
                    |set, prerequisite| {
                        let prerequisite_id = prerequisite.id();
                        match prerequisite.prerequisite_type() {
                            PrerequisiteType::Ability(ability_prerequisite) => {
                                if ability_prerequisite.ability_name == AbilityNameNoSubskill::Craft
                                    && ability_prerequisite.subskill.is_some()
                                {
                                    set.requiring_craft_focus(
                                        ability_prerequisite
                                            .subskill
                                            .as_deref()
                                            .unwrap()
                                            .to_owned(),
                                        ability_prerequisite.dots,
                                        prerequisite_id,
                                    )
                                } else if ability_prerequisite.ability_name
                                    == AbilityNameNoSubskill::MartialArts
                                    && ability_prerequisite.subskill.is_some()
                                {
                                    set.requiring_martial_arts_style(
                                        ability_prerequisite
                                            .subskill
                                            .as_deref()
                                            .unwrap()
                                            .to_owned(),
                                        ability_prerequisite.dots,
                                        prerequisite_id,
                                    )
                                } else {
                                    set.requiring_ability(
                                        ability_prerequisite.ability_name,
                                        ability_prerequisite.dots,
                                        prerequisite_id,
                                    )
                                }
                            }
                            PrerequisiteType::Attribute(attribute_prerequisite) => set
                                .requiring_attribute(
                                    attribute_prerequisite.attribute_name,
                                    attribute_prerequisite.dots,
                                    prerequisite_id,
                                ),
                            PrerequisiteType::Essence(dots) => {
                                set.requiring_essence_rating(*dots, prerequisite_id)
                            }
                            PrerequisiteType::Charm(charm_id) => {
                                set.requiring_charm(*charm_id, prerequisite_id)
                            }
                            PrerequisiteType::ExaltType(exalt_type_prerequisite) => {
                                match exalt_type_prerequisite {
                                    ExaltTypePrerequisite::Solar => {
                                        set.requiring_solar(prerequisite_id)
                                    }
                                    ExaltTypePrerequisite::Lunar => {
                                        set.requiring_lunar(prerequisite_id)
                                    }
                                    ExaltTypePrerequisite::DragonBlooded => {
                                        set.requiring_dragon_blooded(prerequisite_id)
                                    }
                                    ExaltTypePrerequisite::Spirit => {
                                        set.requiring_spirit(false, prerequisite_id)
                                    }
                                    ExaltTypePrerequisite::SpiritOrEclipse => {
                                        set.requiring_spirit(true, prerequisite_id)
                                    }
                                }
                            }
                        }
                    },
                )
                .build()
        })
        .collect()
}
