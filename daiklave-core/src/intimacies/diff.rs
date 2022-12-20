use std::collections::HashMap;

use crate::{id::Id, intimacies::Intimacy};

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
