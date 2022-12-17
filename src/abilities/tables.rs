use std::collections::HashMap;

use crate::abilities::AbilityNameNoSubskill;
use crate::character::CharacterBuilder;
use eyre::{eyre, Report, Result, WrapErr};
use sqlx::postgres::PgHasArrayType;

use super::AbilityNameVanilla;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAME", rename_all = "UPPERCASE")]
pub enum AbilityNamePostgres {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Dodge,
    Integrity,
    Investigation,
    Larceny,
    Linguistics,
    Lore,
    Medicine,
    Melee,
    Occult,
    Performance,
    Presence,
    Resistance,
    Ride,
    Sail,
    Socialize,
    Stealth,
    Survival,
    Thrown,
    War,
}

impl PgHasArrayType for AbilityNamePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ABILITYNAME")
    }
}

impl From<AbilityNamePostgres> for AbilityNameNoSubskill {
    fn from(ability_name_postgres: AbilityNamePostgres) -> Self {
        match ability_name_postgres {
            AbilityNamePostgres::Archery => Self::Archery,
            AbilityNamePostgres::Athletics => Self::Athletics,
            AbilityNamePostgres::Awareness => Self::Awareness,
            AbilityNamePostgres::Brawl => Self::Brawl,
            AbilityNamePostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNamePostgres::Dodge => Self::Dodge,
            AbilityNamePostgres::Integrity => Self::Integrity,
            AbilityNamePostgres::Investigation => Self::Investigation,
            AbilityNamePostgres::Larceny => Self::Larceny,
            AbilityNamePostgres::Linguistics => Self::Linguistics,
            AbilityNamePostgres::Lore => Self::Lore,
            AbilityNamePostgres::Medicine => Self::Medicine,
            AbilityNamePostgres::Melee => Self::Melee,
            AbilityNamePostgres::Occult => Self::Occult,
            AbilityNamePostgres::Performance => Self::Performance,
            AbilityNamePostgres::Presence => Self::Presence,
            AbilityNamePostgres::Resistance => Self::Resistance,
            AbilityNamePostgres::Ride => Self::Ride,
            AbilityNamePostgres::Sail => Self::Sail,
            AbilityNamePostgres::Socialize => Self::Socialize,
            AbilityNamePostgres::Stealth => Self::Stealth,
            AbilityNamePostgres::Survival => Self::Survival,
            AbilityNamePostgres::Thrown => Self::Thrown,
            AbilityNamePostgres::War => Self::War,
        }
    }
}

impl From<AbilityNamePostgres> for AbilityNameVanilla {
    fn from(ability_name_postgres: AbilityNamePostgres) -> Self {
        match ability_name_postgres {
            AbilityNamePostgres::Archery => Self::Archery,
            AbilityNamePostgres::Athletics => Self::Athletics,
            AbilityNamePostgres::Awareness => Self::Awareness,
            AbilityNamePostgres::Brawl => Self::Brawl,
            AbilityNamePostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNamePostgres::Dodge => Self::Dodge,
            AbilityNamePostgres::Integrity => Self::Integrity,
            AbilityNamePostgres::Investigation => Self::Investigation,
            AbilityNamePostgres::Larceny => Self::Larceny,
            AbilityNamePostgres::Linguistics => Self::Linguistics,
            AbilityNamePostgres::Lore => Self::Lore,
            AbilityNamePostgres::Medicine => Self::Medicine,
            AbilityNamePostgres::Melee => Self::Melee,
            AbilityNamePostgres::Occult => Self::Occult,
            AbilityNamePostgres::Performance => Self::Performance,
            AbilityNamePostgres::Presence => Self::Presence,
            AbilityNamePostgres::Resistance => Self::Resistance,
            AbilityNamePostgres::Ride => Self::Ride,
            AbilityNamePostgres::Sail => Self::Sail,
            AbilityNamePostgres::Socialize => Self::Socialize,
            AbilityNamePostgres::Stealth => Self::Stealth,
            AbilityNamePostgres::Survival => Self::Survival,
            AbilityNamePostgres::Thrown => Self::Thrown,
            AbilityNamePostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanilla> for AbilityNamePostgres {
    fn from(ability_name: AbilityNameVanilla) -> Self {
        match ability_name {
            AbilityNameVanilla::Archery => Self::Archery,
            AbilityNameVanilla::Athletics => Self::Athletics,
            AbilityNameVanilla::Awareness => Self::Awareness,
            AbilityNameVanilla::Brawl => Self::Brawl,
            AbilityNameVanilla::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanilla::Dodge => Self::Dodge,
            AbilityNameVanilla::Integrity => Self::Integrity,
            AbilityNameVanilla::Investigation => Self::Investigation,
            AbilityNameVanilla::Larceny => Self::Larceny,
            AbilityNameVanilla::Linguistics => Self::Linguistics,
            AbilityNameVanilla::Lore => Self::Lore,
            AbilityNameVanilla::Medicine => Self::Medicine,
            AbilityNameVanilla::Melee => Self::Melee,
            AbilityNameVanilla::Occult => Self::Occult,
            AbilityNameVanilla::Performance => Self::Performance,
            AbilityNameVanilla::Presence => Self::Presence,
            AbilityNameVanilla::Resistance => Self::Resistance,
            AbilityNameVanilla::Ride => Self::Ride,
            AbilityNameVanilla::Sail => Self::Sail,
            AbilityNameVanilla::Socialize => Self::Socialize,
            AbilityNameVanilla::Stealth => Self::Stealth,
            AbilityNameVanilla::Survival => Self::Survival,
            AbilityNameVanilla::Thrown => Self::Thrown,
            AbilityNameVanilla::War => Self::War,
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "abilities")]
pub struct AbilityRow {
    pub character_id: i32,
    pub name: AbilityNamePostgres,
    pub dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub character_id: i32,
    pub name: AbilityNamePostgres,
    pub specialty: String,
}

impl PgHasArrayType for SpecialtyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_specialties")
    }
}

impl CharacterBuilder {
    fn apply_ability_with_specialties_rows(
        self,
        ability_row: AbilityRow,
        specialty_rows: Vec<SpecialtyRow>,
    ) -> Result<Self> {
        let dots: u8 = ability_row
            .dots
            .try_into()
            .wrap_err_with(|| format!("Invalid number of dots: {}", ability_row.dots))?;


        let ability_name = ability_row.name.try_into().wrap_err_with(|| {
            format!("Could not decode ability name: {:?}", ability_row.name)
        })?;
        specialty_rows.into_iter().fold(
            Ok(self.with_ability(ability_name, dots).wrap_err_with(|| {
                format!(
                    "Could not set ability name {:?} to have dots {}",
                    ability_name, dots
                )
            })?),
            |character_result, specialty_row| {
                character_result.and_then(|character| {
                    character.with_specialty(ability_name, specialty_row.specialty)
                })
            },
        )
    }

    pub(crate) fn apply_abilities_and_specialties_rows(
        self,
        abilities_rows: Vec<AbilityRow>,
        specialty_rows: Option<Vec<SpecialtyRow>>,
    ) -> Result<Self> {
        let mut abilities_hashmap =
            abilities_rows
                .into_iter()
                .fold(HashMap::new(), |mut map, ability| {
                    map.insert(ability.id, (ability, Vec::<SpecialtyRow>::new()));
                    map
                });

        if let Some(specialties) = specialty_rows {
            specialties.into_iter().fold(
                Ok(&mut abilities_hashmap),
                |map: Result<&mut HashMap<i32, (AbilityRow, Vec<SpecialtyRow>)>, eyre::Report>,
                 specialty: SpecialtyRow| {
                    map.and_then(|m| {
                        m.get_mut(&specialty.ability_id)
                            .ok_or_else(|| eyre!("ability {} not found", specialty.ability_id))
                            .map(|tup| tup.1.push(specialty))?;
                        Ok(m)
                    })
                },
            )?;
        };

        abilities_hashmap.into_iter().fold(
            Ok(self),
            |character_result: Result<CharacterBuilder, Report>,
             (_, (ability_row, specialty_rows))| {
                character_result.and_then(|character| {
                    character.apply_ability_with_specialties_rows(ability_row, specialty_rows)
                })
            },
        )
    }
}
