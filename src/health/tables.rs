use sqlx::postgres::PgHasArrayType;

use crate::health::{DamageLevel, WoundPenalty};
use crate::character::{
    CharacterBuilder,
};

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
    Bashing,
    Lethal,
    Aggravated,
}

impl From<DamageLevel> for Option<DamageTypePostgres> {
    fn from(value: DamageLevel) -> Self {
        match value {
            DamageLevel::None => None,
            DamageLevel::Bashing => Some(DamageTypePostgres::Bashing),
            DamageLevel::Lethal => Some(DamageTypePostgres::Lethal),
            DamageLevel::Aggravated => Some(DamageTypePostgres::Aggravated),
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
    pub damage: Option<DamageTypePostgres>,
}

impl CharacterBuilder {
    pub fn apply_health_box_rows(self, health_box_rows: Vec<HealthBoxRow>) -> Self {
        let (mut bashing, mut lethal, mut aggravated) = (0, 0, 0);
        let mut wound_penalties = Vec::new();

        for health_box_row in health_box_rows.into_iter() {
            wound_penalties.push(health_box_row.wound_penalty.into());

            match health_box_row.damage {
                Some(DamageTypePostgres::Bashing) => {
                    bashing += 1;
                }
                Some(DamageTypePostgres::Lethal) => {
                    lethal += 1;
                }
                Some(DamageTypePostgres::Aggravated) => {
                    aggravated += 1;
                }
                None => {}
            }
        }
        self.with_wound_penalties(wound_penalties)
            .with_damage(bashing, lethal, aggravated)
    }
}
