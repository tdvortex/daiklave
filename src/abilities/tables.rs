use crate::abilities::AbilityNameNoSubskill;
use crate::character::CharacterBuilder;
use eyre::{Result, WrapErr};
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
    pub(crate) fn apply_abilities_and_specialties_rows(
        mut self,
        abilities_rows: Vec<AbilityRow>,
        specialty_rows: Option<Vec<SpecialtyRow>>,
    ) -> Result<Self> {
        for row in abilities_rows.into_iter() {
            self = self.with_ability(
                row.name.into(),
                row.dots
                    .try_into()
                    .wrap_err_with(|| format!("Invalid number of dots: {}", row.dots))?,
            );
        }

        if let Some(rows) = specialty_rows {
            for row in rows.into_iter() {
                self = self.with_specialty(row.name.into(), row.specialty)?;
            }
        }

        Ok(self)
    }
}
