use crate::{
    character::traits::merits::{Merit, Merits},
    database::queries::{post_merits_details_transaction, post_new_merits_transaction},
};
use eyre::Result;
use sqlx::{query, Postgres, Transaction};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct MeritDiff {
    insert_merit_templates: Vec<Merit>,
    insert_merit_instance: Vec<(i32, Option<String>)>,
    remove_merit_instances: Vec<i32>,
}

pub fn compare_merits(old_merits: &Merits, new_merits: &Merits) -> MeritDiff {
    let mut diff = MeritDiff::default();

    let mut old_merit_instance_ids: HashSet<i32> = old_merits
        .iter()
        .filter_map(|merit| merit.instance_id())
        .collect();

    for merit in new_merits.iter() {
        if merit.template_id().is_none() {
            diff.insert_merit_templates.push(merit.clone())
        } else if merit.instance_id().is_none() {
            diff.insert_merit_instance.push((
                merit.template_id().unwrap(),
                merit.detail().map(|s| s.to_owned()),
            ))
        } else {
            old_merit_instance_ids.remove(merit.instance_id().as_ref().unwrap());
        }
    }

    diff.remove_merit_instances = old_merit_instance_ids.into_iter().collect();

    diff
}

impl MeritDiff {
    pub async fn save(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        query!(
            "DELETE FROM character_merits
            WHERE character_id = $1::INTEGER AND id IN (SELECT data.id FROM UNNEST($2::INTEGER[]) as data(id))",
            character_id,
            &self.remove_merit_instances as &[i32]
        ).execute(&mut *transaction).await?;

        post_new_merits_transaction(transaction, self.insert_merit_templates, character_id).await?;

        post_merits_details_transaction(transaction, self.insert_merit_instance, character_id)
            .await?;
        Ok(())
    }
}
