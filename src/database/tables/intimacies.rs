use sqlx::postgres::PgHasArrayType;

use crate::character::{
    builder::CharacterBuilder,
    traits::intimacies::{Intimacy, IntimacyLevel, IntimacyType},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYTYPE", rename_all = "UPPERCASE")]
pub enum IntimacyTypePostgres {
    Tie,
    Principle,
}

impl From<IntimacyTypePostgres> for IntimacyType {
    fn from(intimacy_type_postgres: IntimacyTypePostgres) -> Self {
        match intimacy_type_postgres {
            IntimacyTypePostgres::Tie => IntimacyType::Tie,
            IntimacyTypePostgres::Principle => IntimacyType::Principle,
        }
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
    fn from(intimacy_level: IntimacyLevelPostgres) -> Self {
        match intimacy_level {
            IntimacyLevelPostgres::Minor => IntimacyLevel::Minor,
            IntimacyLevelPostgres::Major => IntimacyLevel::Major,
            IntimacyLevelPostgres::Defining => IntimacyLevel::Defining,
        }
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

impl CharacterBuilder {
    pub fn apply_intimacy_rows(&mut self, intimacy_rows: Option<Vec<IntimacyRow>>) -> &mut Self {
        if let Some(rows) = intimacy_rows {
            rows.into_iter().fold(self, |s, intimacy_row| {
                s.with_intimacy(Intimacy {
                    intimacy_level: intimacy_row.level.into(),
                    intimacy_type: intimacy_row.intimacy_type.into(),
                    description: intimacy_row.description,
                })
            })
        } else {
            self
        }
    }
}
