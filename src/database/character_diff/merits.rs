use std::collections::HashSet;

use crate::character::traits::merits::{Merits, Merit};

#[derive(Debug, Default)]
pub struct MeritDiff {
    insert_merit_templates: Vec<Merit>,
    insert_merit_instance: Vec<Merit>,
    remove_merit_instances: Vec<i32>,
}

pub fn compare_merits(old_merits: &Merits, new_merits: &Merits) -> MeritDiff {
    let mut diff = MeritDiff::default();

    let mut old_merit_instance_ids: HashSet<i32> = old_merits.iter().filter_map(|merit| merit.instance_id()).collect();

    for merit in new_merits.iter() {
        if merit.template_id().is_none() {
            diff.insert_merit_templates.push(merit.clone())
        } else if merit.instance_id().is_none() {
            diff.insert_merit_instance.push(merit.clone())
        } else {
            old_merit_instance_ids.remove(merit.instance_id().as_ref().unwrap());
        }
    }

    diff.remove_merit_instances = old_merit_instance_ids.into_iter().collect();
    
    diff
}