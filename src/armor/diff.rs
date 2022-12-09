use eyre::Result;
use sqlx::{query, Postgres, Transaction};
use std::collections::HashSet;

use super::insert::post_armor_transaction;
use super::{Armor, ArmorItem};

pub struct ArmorDiff {
    noop: bool,
    insert_items: Vec<(ArmorItem, bool)>,
    owned_items: Vec<i32>,
    worn_item: Option<i32>,
}

impl Armor {
    pub fn compare_newer(&self, newer: &Armor) -> ArmorDiff {
        let mut diff = ArmorDiff {
            noop: true,
            insert_items: Vec::new(),
            owned_items: Vec::new(),
            worn_item: None,
        };

        let mut new_owned_set = HashSet::new();
        let mut worn_item = None;

        for (_, worn, armor_item) in newer.iter() {
            if let Some(id) = armor_item.id() {
                new_owned_set.insert(id);
                if worn {
                    worn_item = Some(id);
                }
            } else {
                diff.noop = false;
                diff.insert_items.push((armor_item.clone(), worn));
            }
        }

        let mut old_owned_set = HashSet::new();

        for (_, worn, armor_item) in self.iter() {
            if let Some(old_id) = armor_item.id() {
                old_owned_set.insert(old_id);
                if diff.noop && worn && Some(old_id) != worn_item {
                    diff.noop = false;
                }
            }
        }

        if diff.noop && new_owned_set != old_owned_set {
            diff.noop = false;
        }

        if !diff.noop {
            diff.owned_items = new_owned_set.into_iter().collect();
            diff.worn_item = worn_item;
        }

        diff
    }
}

impl ArmorDiff {
    pub async fn save(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if self.noop {
            return Ok(());
        }

        query!(
            "DELETE FROM character_armor
            WHERE character_id = $1
            ",
            character_id
        )
        .execute(&mut *transaction)
        .await?;

        let (new_items, mut new_items_equipped) = self.insert_items.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut new_items, mut new_items_equipped), (item, equipped)| {
                new_items.push(item);
                new_items_equipped.push(equipped);
                (new_items, new_items_equipped)
            },
        );

        let mut new_ids = post_armor_transaction(transaction, new_items).await?;

        let (mut ids, mut ids_equipped) = self.owned_items.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut ids, mut ids_equipped), id| {
                ids.push(id);
                ids_equipped.push(Some(id) == self.worn_item);
                (ids, ids_equipped)
            },
        );

        ids.append(&mut new_ids);
        ids_equipped.append(&mut new_items_equipped);

        query!(
            "INSERT INTO character_armor(character_id, armor_id, worn)
            SELECT
                $1::INTEGER as character_id,
                data.armor_id as armor_id,
                data.worn as worn
            FROM UNNEST($2::INTEGER[], $3::BOOLEAN[]) as data(armor_id, worn)",
            character_id,
            &ids as &[i32],
            &ids_equipped as &[bool]
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}
