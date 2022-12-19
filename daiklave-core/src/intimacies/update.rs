use std::collections::HashMap;

use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::{
    id::Id,
    intimacies::tables::{IntimacyLevelPostgres, IntimacyTypePostgres},
    intimacies::Intimacy,
};

#[derive(Debug, Default)]
pub struct IntimaciesDiff {
    pub new_intimacies: Vec<Intimacy>,
    pub updated_intimacies: Vec<Intimacy>,
    pub deleted_intimacies: Vec<i32>,
}

pub fn compare_intimacies(older: &[Intimacy], newer: &[Intimacy]) -> IntimaciesDiff {
    let mut diff = IntimaciesDiff::default();

    let old_hashmap: HashMap<i32, &Intimacy> = older
        .iter()
        .filter_map(|intimacy| {
            if let Id::Database(id) = intimacy.id {
                Some((id, intimacy))
            } else {
                None
            }
        })
        .collect();

    let new_hashmap: HashMap<i32, &Intimacy> = newer
        .iter()
        .filter_map(|intimacy| {
            if let Id::Database(id) = intimacy.id {
                Some((id, intimacy))
            } else {
                diff.new_intimacies.push(intimacy.clone());
                None
            }
        })
        .collect();

    for id in old_hashmap.keys() {
        if new_hashmap.contains_key(id) {
            if *old_hashmap.get(id).unwrap() != *new_hashmap.get(id).unwrap() {
                diff.updated_intimacies
                    .push((*new_hashmap.get(id).unwrap()).clone());
            }
        } else {
            diff.deleted_intimacies.push(*id);
        }
    }

    diff
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

