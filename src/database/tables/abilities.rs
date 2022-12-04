use std::collections::HashMap;

use sqlx::postgres::PgHasArrayType;

use crate::character::{traits::abilities::AbilityNameNoFocus, builder::CharacterBuilder};
use eyre::{eyre, Report, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAME", rename_all = "UPPERCASE")]
pub enum AbilityNamePostgres {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Craft,
    Dodge,
    Integrity,
    Investigation,
    Larceny,
    Linguistics,
    Lore,
    MartialArts,
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

impl From<AbilityNamePostgres> for AbilityNameNoFocus {
    fn from(ability_name_postgres: AbilityNamePostgres) -> Self {
        match ability_name_postgres {
            AbilityNamePostgres::Archery => Self::Archery,
            AbilityNamePostgres::Athletics => Self::Athletics,
            AbilityNamePostgres::Awareness => Self::Awareness,
            AbilityNamePostgres::Brawl => Self::Brawl,
            AbilityNamePostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNamePostgres::Craft => Self::Craft,
            AbilityNamePostgres::Dodge => Self::Dodge,
            AbilityNamePostgres::Integrity => Self::Integrity,
            AbilityNamePostgres::Investigation => Self::Investigation,
            AbilityNamePostgres::Larceny => Self::Larceny,
            AbilityNamePostgres::Linguistics => Self::Linguistics,
            AbilityNamePostgres::Lore => Self::Lore,
            AbilityNamePostgres::MartialArts => Self::MartialArts,
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "abilities")]
pub struct AbilityRow {
    pub id: i32,
    pub character_id: i32,
    pub name: AbilityNamePostgres,
    pub dots: i16,
    pub subskill: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub id: i32,
    pub ability_id: i32,
    pub specialty: String,
}

impl PgHasArrayType for SpecialtyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_specialties")
    }
}

impl CharacterBuilder {
    fn apply_ability_with_specialties_rows(
        &mut self,
        ability_row: AbilityRow,
        specialty_rows: Vec<SpecialtyRow>,
    ) -> Result<&mut Self> {
        let dots: u8 = ability_row.dots.try_into()?;

        match ability_row.name {
            AbilityNamePostgres::Craft => {
                let craft_focus = ability_row
                    .subskill
                    .ok_or(eyre!("craft abilities must have a focus"))?;
                self.with_craft(craft_focus.as_str(), dots);
                specialty_rows
                    .into_iter()
                    .fold(Ok(self), |character_result, specialty_row| {
                        character_result.and_then(|character| {
                            character
                                .with_craft_specialty(craft_focus.as_str(), specialty_row.specialty)
                        })
                    })
            }
            AbilityNamePostgres::MartialArts => {
                let martial_arts_style = ability_row
                    .subskill
                    .ok_or(eyre!("martial arts abilities must have a style"))?;
                self.with_martial_arts(martial_arts_style.as_str(), dots);
                specialty_rows
                    .into_iter()
                    .fold(Ok(self), |character_result, specialty_row| {
                        character_result.and_then(|character| {
                            character.with_martial_arts_specialty(
                                martial_arts_style.as_str(),
                                specialty_row.specialty,
                            )
                        })
                    })
            }
            other_ability => {
                let ability_name = other_ability.try_into()?;
                self.with_ability(ability_name, dots)?;
                specialty_rows
                    .into_iter()
                    .fold(Ok(self), |character_result, specialty_row| {
                        character_result.and_then(|character| {
                            character.with_specialty(ability_name, specialty_row.specialty)
                        })
                    })
            }
        }
    }

    pub fn apply_abilities_and_specialties_rows(
        &mut self,
        abilities_rows: Vec<AbilityRow>,
        specialty_rows: Option<Vec<SpecialtyRow>>,
    ) -> Result<&mut Self> {
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
            |character_result: Result<&mut CharacterBuilder, Report>,
             (_, (ability_row, specialty_rows))| {
                character_result.and_then(|character| {
                    character.apply_ability_with_specialties_rows(ability_row, specialty_rows)
                })
            },
        )
    }
}
