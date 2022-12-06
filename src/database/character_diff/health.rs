use crate::{
    character::traits::health::Health,
    database::tables::health::{DamageTypePostgres, WoundPenaltyPostgres},
};
use eyre::Result;
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, Default)]
pub struct HealthDiff {
    upserted_boxes: Vec<(i16, WoundPenaltyPostgres, Option<DamageTypePostgres>)>,
    deleted_boxes: Vec<i16>,
}

impl Health {
    pub fn compare_newer(&self, newer: &Self) -> HealthDiff {
        let mut diff = HealthDiff::default();

        let mut old_vec: Vec<(i16, WoundPenaltyPostgres, Option<DamageTypePostgres>)> = self
            .health_boxes()
            .iter()
            .enumerate()
            .take(i16::MAX as usize)
            .map(|(index, health_box)| {
                (
                    index as i16,
                    health_box.wound_penalty().into(),
                    health_box.damage().into(),
                )
            })
            .collect();

        (0..(old_vec.len() - newer.health_boxes().len()))
            .for_each(|_| diff.deleted_boxes.push(old_vec.pop().unwrap().0));

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
                        .push((index as i16, wound_penalty, maybe_damage));
                } else if old_vec[index].1 != wound_penalty || old_vec[index].2 != maybe_damage {
                    diff.upserted_boxes
                        .push((old_vec[index].0, wound_penalty, maybe_damage))
                }
            });

        diff
    }
}

impl HealthDiff {
    pub async fn save(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<&Self> {
        if !self.deleted_boxes.is_empty() {
            query!(
                "
                DELETE FROM health_boxes
                WHERE health_boxes.character_id = $1::INTEGER
                AND health_boxes.position IN (SELECT * FROM UNNEST($2::SMALLINT[]))
                ",
                character_id,
                &self.deleted_boxes as &[i16]
            )
            .execute(&mut *transaction)
            .await?;
        }

        if !self.upserted_boxes.is_empty() {
            let upserted_positions = self
                .upserted_boxes
                .iter()
                .map(|x| x.0)
                .collect::<Vec<i16>>();
            let upserted_wound_penalties = self
                .upserted_boxes
                .iter()
                .map(|x| x.1)
                .collect::<Vec<WoundPenaltyPostgres>>();
            let upserted_damages = self
                .upserted_boxes
                .iter()
                .map(|x| x.2)
                .collect::<Vec<Option<DamageTypePostgres>>>();
            query!(
                "
                INSERT INTO health_boxes(character_id, position, wound_penalty, current_damage)
                SELECT
                    $1::INTEGER as character_id,
                    data.position as position,
                    data.wound_penalty as wound_penalty,
                    data.current_damage as current_damage
                FROM UNNEST($2::SMALLINT[], $3::WOUNDPENALTY[], $4::DAMAGETYPE[]) as data(position, wound_penalty, current_damage)
                ON CONFLICT (character_id, position)
                DO UPDATE SET (wound_penalty, current_damage) = (EXCLUDED.wound_penalty, EXCLUDED.current_damage)
                ",
                character_id,
                &upserted_positions as &[i16],
                &upserted_wound_penalties as &[WoundPenaltyPostgres],
                &upserted_damages as &[Option<DamageTypePostgres>]
            ).execute(&mut *transaction).await?;
        }

        Ok(self)
    }
}
