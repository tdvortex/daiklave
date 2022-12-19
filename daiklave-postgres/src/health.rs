use daiklave_core::{
    character::CharacterBuilder,
    health::{DamageLevel, HealthDiff, WoundPenalty},
};
use eyre::{Result, WrapErr};
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WOUNDPENALTY", rename_all = "UPPERCASE")]
pub enum WoundPenaltyPostgres {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
}

impl From<WoundPenalty> for WoundPenaltyPostgres {
    fn from(value: WoundPenalty) -> Self {
        match value {
            WoundPenalty::Zero => Self::Zero,
            WoundPenalty::MinusOne => Self::MinusOne,
            WoundPenalty::MinusTwo => Self::MinusTwo,
            WoundPenalty::MinusFour => Self::MinusFour,
            WoundPenalty::Incapacitated => Self::Incapacitated,
        }
    }
}

impl From<WoundPenaltyPostgres> for WoundPenalty {
    fn from(value: WoundPenaltyPostgres) -> Self {
        match value {
            WoundPenaltyPostgres::Zero => Self::Zero,
            WoundPenaltyPostgres::MinusOne => Self::MinusOne,
            WoundPenaltyPostgres::MinusTwo => Self::MinusTwo,
            WoundPenaltyPostgres::MinusFour => Self::MinusFour,
            WoundPenaltyPostgres::Incapacitated => Self::Incapacitated,
        }
    }
}

impl PgHasArrayType for WoundPenaltyPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WOUNDPENALTY")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "DAMAGETYPE", rename_all = "UPPERCASE")]
pub enum DamageTypePostgres {
    None,
    Bashing,
    Lethal,
    Aggravated,
}

impl From<DamageLevel> for DamageTypePostgres {
    fn from(value: DamageLevel) -> Self {
        match value {
            DamageLevel::None => DamageTypePostgres::None,
            DamageLevel::Bashing => DamageTypePostgres::Bashing,
            DamageLevel::Lethal => DamageTypePostgres::Lethal,
            DamageLevel::Aggravated => DamageTypePostgres::Aggravated,
        }
    }
}

impl PgHasArrayType for DamageTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_DAMAGETYPE")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "health_boxes")]
pub struct HealthBoxRow {
    pub character_id: i32,
    pub position: i16,
    pub wound_penalty: WoundPenaltyPostgres,
    pub damage: DamageTypePostgres,
}

pub fn apply_health_box_rows(
    builder: CharacterBuilder,
    health_box_rows: Vec<HealthBoxRow>,
) -> CharacterBuilder {
    let (mut bashing, mut lethal, mut aggravated) = (0, 0, 0);
    let mut wound_penalties = Vec::new();

    for health_box_row in health_box_rows.into_iter() {
        wound_penalties.push(health_box_row.wound_penalty.into());

        match health_box_row.damage {
            DamageTypePostgres::Bashing => {
                bashing += 1;
            }
            DamageTypePostgres::Lethal => {
                lethal += 1;
            }
            DamageTypePostgres::Aggravated => {
                aggravated += 1;
            }
            DamageTypePostgres::None => {}
        }
    }
    builder
        .with_wound_penalties(wound_penalties)
        .with_damage(bashing, lethal, aggravated)
}

pub async fn update_health(
    health_diff: HealthDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if !health_diff.deleted_boxes.is_empty() {
        let mut deleted_positions = Vec::<i16>::new();
        for position in health_diff.deleted_boxes.into_iter() {
            deleted_positions.push(
                position
                    .try_into()
                    .wrap_err("Number of health boxes overflows i16")?,
            );
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
