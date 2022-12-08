use std::collections::HashSet;

use crate::character::traits::armor::{Armor, ArmorItem};

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