use super::tables::{DamageTypePostgres, WoundPenaltyPostgres};
use super::{Health, WoundPenalty, DamageLevel};
use eyre::{WrapErr, Result};
use sqlx::{query, Postgres, Transaction};

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


pub async fn update_health(
    health_diff: HealthDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if !health_diff.deleted_boxes.is_empty() {
        let mut deleted_positions = Vec::<i16>::new();
        for position in health_diff.deleted_boxes.into_iter() {
            deleted_positions.push(position.try_into().wrap_err("Number of health boxes overflows i16")?);
        }

        query!(
            "
            DELETE FROM health_boxes
            WHERE health_boxes.character_id = $1::INTEGER
            AND health_boxes.position IN (SELECT * FROM UNNEST($2::SMALLINT[]))
            ",
            character_id,
            &deleted_positions as &[i16]
        )
        .execute(&mut *transaction)
        .await?;
    }

    if !health_diff.upserted_boxes.is_empty() {
        let upserted_positions = health_diff
            .upserted_boxes
            .iter()
            .map(|x| x.0.try_into().unwrap())
            .collect::<Vec<i16>>();
        let upserted_wound_penalties = health_diff
            .upserted_boxes
            .iter()
            .map(|x| x.1.into())
            .collect::<Vec<WoundPenaltyPostgres>>();
        let upserted_damages = health_diff
            .upserted_boxes
            .iter()
            .map(|x| x.2.map(|dmg| dmg.into()))
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

    Ok(())
}

