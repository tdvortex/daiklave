use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::DamageLevel;

pub struct TakeDamage {
    level: DamageLevel,
    amount: NonZeroU8,
}

impl TakeDamage {
    pub fn new(level: DamageLevel, amount: NonZeroU8) -> Self {
        Self {
            level,
            amount,
        }
    }
}

impl From<TakeDamage> for CharacterMutation {
    fn from(take_damage: TakeDamage) -> Self {
        CharacterMutation::TakeDamage(take_damage)
    }
}