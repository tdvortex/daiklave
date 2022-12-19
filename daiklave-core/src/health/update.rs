
use super::{Health, WoundPenalty, DamageLevel};

#[derive(Debug, Default)]
pub struct HealthDiff {
    pub upserted_boxes: Vec<(usize, WoundPenalty, Option<DamageLevel>)>,
    pub deleted_boxes: Vec<usize>,
}

impl Health {
    pub fn compare_newer(&self, newer: &Self) -> HealthDiff {
        let mut diff = HealthDiff::default();

        let mut old_vec: Vec<(usize, WoundPenalty, Option<DamageLevel>)> = self
            .health_boxes()
            .iter()
            .enumerate()
            .take(i16::MAX as usize)
            .map(|(index, health_box)| {
                (
                    index as usize,
                    health_box.wound_penalty().into(),
                    health_box.damage().into(),
                )
            })
            .collect();

        if old_vec.len() > newer.health_boxes.len() {
            (0..(old_vec.len() - newer.health_boxes().len()))
                .for_each(|_| diff.deleted_boxes.push(old_vec.pop().unwrap().0));
        }

        newer
            .health_boxes()
            .iter()
            .enumerate()
            .take(i16::MAX as usize)
            .map(|(index, health_box)| {
                (
                    index,
                    health_box.wound_penalty().into(),
                    health_box.damage().into(),
                )
            })
            .for_each(|(index, wound_penalty, maybe_damage)| {
                if index >= old_vec.len() {
                    diff.upserted_boxes
                        .push((index, wound_penalty, maybe_damage));
                } else if old_vec[index].1 != wound_penalty || old_vec[index].2 != maybe_damage {
                    diff.upserted_boxes
                        .push((old_vec[index].0, wound_penalty, maybe_damage))
                }
            });

        diff
    }
}