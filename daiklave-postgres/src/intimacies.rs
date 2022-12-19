use daiklave_core::{
    character::CharacterBuilder,
    id::Id,
    intimacies::{IntimaciesDiff, Intimacy, IntimacyLevel, IntimacyType},
};
use eyre::Result;
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

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

pub fn apply_intimacy_rows(
    builder: CharacterBuilder,
    intimacy_rows: Option<Vec<IntimacyRow>>,
) -> CharacterBuilder {
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

async fn add_new_intimacies(
    intimacies_diff: &IntimaciesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if intimacies_diff.new_intimacies.is_empty() {
        return Ok(());
    }

    let mut types: Vec<IntimacyTypePostgres> = Vec::new();
    let mut levels: Vec<IntimacyLevelPostgres> = Vec::new();
    let mut descriptions: Vec<&str> = Vec::new();
    for intimacy in intimacies_diff.new_intimacies.iter() {
        types.push(intimacy.intimacy_type.into());
        levels.push(intimacy.intimacy_level.into());
        descriptions.push(intimacy.description.as_str());
    }

    query!(
        "
        INSERT INTO intimacies(character_id, intimacy_type, level, description)
        SELECT
            $1::INTEGER as character_id,
            data.intimacy_type,
            data.level,
            data.description
        FROM UNNEST($2::INTIMACYTYPE[], $3::INTIMACYLEVEL[], $4::TEXT[]) as data(intimacy_type, level, description)
        ",
        character_id,
        &types as &[IntimacyTypePostgres],
        &levels as &[IntimacyLevelPostgres],
        &descriptions as &[&str],
    ).execute(&mut *transaction).await?;

    Ok(())
}

async fn delete_intimacies(
    intimacies_diff: &IntimaciesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if intimacies_diff.deleted_intimacies.is_empty() {
        return Ok(());
    }

    query!(
        "DELETE FROM intimacies
        WHERE character_id = $1
        AND id IN (SELECT * FROM UNNEST($2::INTEGER[]))",
        character_id,
        &intimacies_diff.deleted_intimacies as &[i32],
    )
    .execute(&mut *transaction)
    .await?;

    Ok(())
}

async fn modify_intimacies(
    intimacies_diff: &IntimaciesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if intimacies_diff.updated_intimacies.is_empty() {
        return Ok(());
    }

    let mut ids: Vec<i32> = Vec::new();
    let mut types: Vec<IntimacyTypePostgres> = Vec::new();
    let mut levels: Vec<IntimacyLevelPostgres> = Vec::new();
    let mut descriptions: Vec<&str> = Vec::new();
    for intimacy in intimacies_diff.updated_intimacies.iter() {
        if let Id::Database(id) = intimacy.id {
            ids.push(id);
            types.push(intimacy.intimacy_type.into());
            levels.push(intimacy.intimacy_level.into());
            descriptions.push(intimacy.description.as_str());
        }
    }

    query!(
        "UPDATE intimacies SET 
            intimacy_type = new.intimacy_type, 
            level = new.intimacy_level, 
            description = new.description
        FROM UNNEST($2::INTEGER[], $3::INTIMACYTYPE[], $4::INTIMACYLEVEL[], $5::TEXT[]) as new(id, intimacy_type, intimacy_level, description)
        WHERE intimacies.character_id = $1::INTEGER AND intimacies.id = new.id",
        character_id,
        &ids as &[i32],
        &types as &[IntimacyTypePostgres],
        &levels as &[IntimacyLevelPostgres],
        &descriptions as &[&str],
    ).execute(&mut *transaction).await?;

    Ok(())
}

pub async fn update_intimacies(
    intimacies_diff: IntimaciesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    delete_intimacies(&intimacies_diff, transaction, character_id).await?;
    modify_intimacies(&intimacies_diff, transaction, character_id).await?;
    add_new_intimacies(&intimacies_diff, transaction, character_id).await?;
    Ok(())
}
