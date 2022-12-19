use crate::id::Id;

use super::Merit;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct MeritDiff {
    pub insert_merit_templates: Vec<Merit>,
    pub insert_merit_instance: Vec<(i32, u8, Option<String>)>,
    pub remove_merit_instances: Vec<i32>,
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
                merit.dots(),
                merit.detail().map(|s| s.to_owned()),
            ))
        } else {
            old_merit_instance_ids.remove(&*merit.instance_id());
        }
    }

    diff.remove_merit_instances = old_merit_instance_ids.into_iter().collect();

    diff
}
