use daiklave_core::{intimacies::{IntimacyLevel, IntimacyType, Intimacy}, character::CharacterBuilder, id::Id};
use sqlx::postgres::PgHasArrayType;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYTYPE", rename_all = "UPPERCASE")]
pub enum IntimacyTypePostgres {
    Tie,
    Principle,
}

impl From<IntimacyTypePostgres> for IntimacyType {
    fn from(intimacy_type_postgres: IntimacyTypePostgres) -> Self {
        match intimacy_type_postgres {
            IntimacyTypePostgres::Tie => Self::Tie,
            IntimacyTypePostgres::Principle => Self::Principle,
        }
    }
}

impl From<IntimacyType> for IntimacyTypePostgres {
    fn from(intimacy_type: IntimacyType) -> Self {
        match intimacy_type {
            IntimacyType::Tie => Self::Tie,
            IntimacyType::Principle => Self::Principle,
        }
    }
}

impl PgHasArrayType for IntimacyTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_INTIMACYTYPE")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYLEVEL", rename_all = "UPPERCASE")]
pub enum IntimacyLevelPostgres {
    Minor,
    Major,
    Defining,
}

impl From<IntimacyLevelPostgres> for IntimacyLevel {
    fn from(intimacy_level_postgres: IntimacyLevelPostgres) -> Self {
        match intimacy_level_postgres {
            IntimacyLevelPostgres::Minor => Self::Minor,
            IntimacyLevelPostgres::Major => Self::Major,
            IntimacyLevelPostgres::Defining => Self::Defining,
        }
    }
}

impl From<IntimacyLevel> for IntimacyLevelPostgres {
    fn from(intimacy_level: IntimacyLevel) -> Self {
        match intimacy_level {
            IntimacyLevel::Minor => Self::Minor,
            IntimacyLevel::Major => Self::Major,
            IntimacyLevel::Defining => Self::Defining,
        }
    }
}

impl PgHasArrayType for IntimacyLevelPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_INTIMACYLEVEL")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "intimacies")]
pub struct IntimacyRow {
    pub id: i32,
    pub character_id: i32,
    pub intimacy_type: IntimacyTypePostgres,
    pub level: IntimacyLevelPostgres,
    pub description: String,
}

impl PgHasArrayType for IntimacyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_intimacies")
    }
}

pub fn apply_intimacy_rows(builder: CharacterBuilder, intimacy_rows: Option<Vec<IntimacyRow>>) -> CharacterBuilder {
    if let Some(rows) = intimacy_rows {
        rows.into_iter().fold(builder, |s, intimacy_row| {
            s.with_intimacy(Intimacy {
                id: Id::Database(intimacy_row.id),
                intimacy_level: intimacy_row.level.into(),
                intimacy_type: intimacy_row.intimacy_type.into(),
                description: intimacy_row.description,
            })
        })
    } else {
        builder
    }
}