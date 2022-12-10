use std::collections::HashMap;

use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::{
    intimacies::tables::{IntimacyLevelPostgres, IntimacyTypePostgres},
    intimacies::{Intimacy},
};

#[derive(Debug, Default)]
pub struct IntimaciesDiff {
    new_intimacies: Vec<Intimacy>,
    updated_intimacies: Vec<Intimacy>,
    deleted_intimacies: Vec<i32>,
}

pub fn compare_intimacies(older: &[Intimacy], newer: &[Intimacy]) -> IntimaciesDiff {
    let mut diff = IntimaciesDiff::default();

    let old_hashmap: HashMap<i32, &Intimacy> = older
        .iter()
        .filter_map(|intimacy| {
            let id = intimacy.id?;
            Some((id, intimacy))
        })
        .collect();

    let new_hashmap: HashMap<i32, &Intimacy> = newer
        .iter()
        .filter_map(|intimacy| {
            if let Some(id) = intimacy.id {
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

impl IntimaciesDiff {
    async fn add_new_intimacies(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<&Self> {
        if self.new_intimacies.is_empty() {
            return Ok(self);
        }

        let mut types: Vec<IntimacyTypePostgres> = Vec::new();
        let mut levels: Vec<IntimacyLevelPostgres> = Vec::new();
        let mut descriptions: Vec<&str> = Vec::new();
        for intimacy in self.new_intimacies.iter() {
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

        Ok(self)
    }

    async fn delete_intimacies(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<&Self> {
        if self.deleted_intimacies.is_empty() {
            return Ok(self);
        }

        query!(
            "DELETE FROM intimacies
            WHERE character_id = $1
            AND id IN (SELECT * FROM UNNEST($2::INTEGER[]))",
            character_id,
            &self.deleted_intimacies as &[i32],
        )
        .execute(&mut *transaction)
        .await?;

        Ok(self)
    }

    async fn update_intimacies(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<&Self> {
        if self.updated_intimacies.is_empty() {
            return Ok(self);
        }

        let mut ids: Vec<i32> = Vec::new();
        let mut types: Vec<IntimacyTypePostgres> = Vec::new();
        let mut levels: Vec<IntimacyLevelPostgres> = Vec::new();
        let mut descriptions: Vec<&str> = Vec::new();
        for intimacy in self.updated_intimacies.iter() {
            ids.push(intimacy.id.unwrap());
            types.push(intimacy.intimacy_type.into());
            levels.push(intimacy.intimacy_level.into());
            descriptions.push(intimacy.description.as_str());
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

        Ok(self)
    }

    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        self.delete_intimacies(transaction, character_id).await?;
        self.update_intimacies(transaction, character_id).await?;
        self.add_new_intimacies(transaction, character_id).await?;
        Ok(())
    }
}
