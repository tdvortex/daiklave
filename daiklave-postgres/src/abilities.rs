use daiklave_core::{
    abilities::{AbilityNameNoSubskill, AbilityNameVanilla},
    character::CharacterBuilder,
};
use eyre::{Result, WrapErr};
use sqlx::postgres::PgHasArrayType;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAMEVANILLA", rename_all = "UPPERCASE")]
pub enum AbilityNameVanillaPostgres {
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

impl PgHasArrayType for AbilityNameVanillaPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ABILITYNAMEVANILLA")
    }
}

impl From<AbilityNameVanillaPostgres> for AbilityNameNoSubskill {
    fn from(ability_name_postgres: AbilityNameVanillaPostgres) -> Self {
        match ability_name_postgres {
            AbilityNameVanillaPostgres::Archery => Self::Archery,
            AbilityNameVanillaPostgres::Athletics => Self::Athletics,
            AbilityNameVanillaPostgres::Awareness => Self::Awareness,
            AbilityNameVanillaPostgres::Brawl => Self::Brawl,
            AbilityNameVanillaPostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanillaPostgres::Dodge => Self::Dodge,
            AbilityNameVanillaPostgres::Integrity => Self::Integrity,
            AbilityNameVanillaPostgres::Investigation => Self::Investigation,
            AbilityNameVanillaPostgres::Larceny => Self::Larceny,
            AbilityNameVanillaPostgres::Linguistics => Self::Linguistics,
            AbilityNameVanillaPostgres::Lore => Self::Lore,
            AbilityNameVanillaPostgres::Medicine => Self::Medicine,
            AbilityNameVanillaPostgres::Melee => Self::Melee,
            AbilityNameVanillaPostgres::Occult => Self::Occult,
            AbilityNameVanillaPostgres::Performance => Self::Performance,
            AbilityNameVanillaPostgres::Presence => Self::Presence,
            AbilityNameVanillaPostgres::Resistance => Self::Resistance,
            AbilityNameVanillaPostgres::Ride => Self::Ride,
            AbilityNameVanillaPostgres::Sail => Self::Sail,
            AbilityNameVanillaPostgres::Socialize => Self::Socialize,
            AbilityNameVanillaPostgres::Stealth => Self::Stealth,
            AbilityNameVanillaPostgres::Survival => Self::Survival,
            AbilityNameVanillaPostgres::Thrown => Self::Thrown,
            AbilityNameVanillaPostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanillaPostgres> for AbilityNameVanilla {
    fn from(ability_name_postgres: AbilityNameVanillaPostgres) -> Self {
        match ability_name_postgres {
            AbilityNameVanillaPostgres::Archery => Self::Archery,
            AbilityNameVanillaPostgres::Athletics => Self::Athletics,
            AbilityNameVanillaPostgres::Awareness => Self::Awareness,
            AbilityNameVanillaPostgres::Brawl => Self::Brawl,
            AbilityNameVanillaPostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanillaPostgres::Dodge => Self::Dodge,
            AbilityNameVanillaPostgres::Integrity => Self::Integrity,
            AbilityNameVanillaPostgres::Investigation => Self::Investigation,
            AbilityNameVanillaPostgres::Larceny => Self::Larceny,
            AbilityNameVanillaPostgres::Linguistics => Self::Linguistics,
            AbilityNameVanillaPostgres::Lore => Self::Lore,
            AbilityNameVanillaPostgres::Medicine => Self::Medicine,
            AbilityNameVanillaPostgres::Melee => Self::Melee,
            AbilityNameVanillaPostgres::Occult => Self::Occult,
            AbilityNameVanillaPostgres::Performance => Self::Performance,
            AbilityNameVanillaPostgres::Presence => Self::Presence,
            AbilityNameVanillaPostgres::Resistance => Self::Resistance,
            AbilityNameVanillaPostgres::Ride => Self::Ride,
            AbilityNameVanillaPostgres::Sail => Self::Sail,
            AbilityNameVanillaPostgres::Socialize => Self::Socialize,
            AbilityNameVanillaPostgres::Stealth => Self::Stealth,
            AbilityNameVanillaPostgres::Survival => Self::Survival,
            AbilityNameVanillaPostgres::Thrown => Self::Thrown,
            AbilityNameVanillaPostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanilla> for AbilityNameVanillaPostgres {
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
    pub name: AbilityNameVanillaPostgres,
    pub dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub character_id: i32,
    pub name: AbilityNameVanillaPostgres,
    pub specialty: String,
}

impl PgHasArrayType for SpecialtyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_specialties")
    }
}

pub fn apply_abilities_and_specialties_rows(
    mut builder: CharacterBuilder,
    abilities_rows: Vec<AbilityRow>,
    specialty_rows: Option<Vec<SpecialtyRow>>,
) -> Result<CharacterBuilder> {
    for row in abilities_rows.into_iter() {
        builder = builder.with_ability(
            row.name.into(),
            row.dots
                .try_into()
                .wrap_err_with(|| format!("Invalid number of dots: {}", row.dots))?,
        );
    }

    if let Some(rows) = specialty_rows {
        for row in rows.into_iter() {
            builder = builder.with_specialty(row.name.into(), row.specialty)?;
        }
    }

    Ok(builder)
}
