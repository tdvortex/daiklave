use crate::id::Id;

use super::create::{create_new_merits_transaction, post_merits_details_transaction};
use super::Merit;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct MeritDiff {
    insert_merit_templates: Vec<Merit>,
    insert_merit_instance: Vec<(i32, i16, Option<String>)>,
    remove_merit_instances: Vec<i32>,
}

pub fn compare_merits(old_merits: &[Merit], new_merits: &[Merit]) -> MeritDiff {
    let mut diff = MeritDiff::default();

    let mut old_merit_instance_ids: HashSet<i32> = old_merits
        .iter()
        .filter_map(|merit| {
            if let Id::Database(id) = merit.instance_id() {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    for merit in new_merits.iter() {
        if merit.template_id().is_placeholder() {
            diff.insert_merit_templates.push(merit.clone())
        } else if merit.instance_id().is_placeholder() {
            diff.insert_merit_instance.push((
                *merit.template_id(),
                merit.dots() as i16,
                merit.detail().map(|s| s.to_owned()),
            ))
        } else {
            old_merit_instance_ids.remove(&*merit.instance_id());
        }
    }

    diff.remove_merit_instances = old_merit_instance_ids.into_iter().collect();

    diff
}

impl MeritDiff {
    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if !self.remove_merit_instances.is_empty() {
            query!(
                "DELETE FROM character_merits
                WHERE character_id = $1::INTEGER AND id IN (SELECT data.id FROM UNNEST($2::INTEGER[]) as data(id))",
                character_id,
                &self.remove_merit_instances as &[i32]
            ).execute(&mut *transaction).await.wrap_err("Database error attempting to delete character merits")?;
        }

        create_new_merits_transaction(transaction, self.insert_merit_templates, character_id)
            .await
            .wrap_err("Error attempting to create new merits")?;

        post_merits_details_transaction(transaction, self.insert_merit_instance, character_id)
            .await
            .wrap_err_with(|| {
                format!(
                    "Error attempting to assign merits to character {}",
                    character_id
                )
            })?;
        Ok(())
    }
}
