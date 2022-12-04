use crate::character::builder::CharacterBuilder;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WOUNDPENALTY", rename_all = "UPPERCASE")]
pub enum WoundPenaltyPostgres {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "DAMAGETYPE", rename_all = "UPPERCASE")]
pub enum DamageTypePostgres {
    Bashing,
    Lethal,
    Aggravated,
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
    pub fn apply_health_box_rows(&mut self, health_box_rows: Vec<HealthBoxRow>) -> &mut Self {
        use crate::character::traits::health::WoundPenalty;
        let (mut bashing, mut lethal, mut aggravated) = (0, 0, 0);
        let mut wound_penalties = Vec::new();

        for health_box_row in health_box_rows.into_iter() {
            wound_penalties.push(match health_box_row.wound_penalty {
                WoundPenaltyPostgres::Zero => WoundPenalty::Zero,
                WoundPenaltyPostgres::MinusOne => WoundPenalty::MinusOne,
                WoundPenaltyPostgres::MinusTwo => WoundPenalty::MinusTwo,
                WoundPenaltyPostgres::MinusFour => WoundPenalty::MinusFour,
                WoundPenaltyPostgres::Incapacitated => WoundPenalty::Incapacitated,
            });

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
        self.with_wound_penalties(wound_penalties);
        self.with_damage(bashing, lethal, aggravated);
        self
    }
}
