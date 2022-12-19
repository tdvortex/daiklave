use daiklave_core::{
    character::CharacterBuilder,
    health::{DamageLevel, WoundPenalty},
};
use sqlx::postgres::PgHasArrayType;

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
