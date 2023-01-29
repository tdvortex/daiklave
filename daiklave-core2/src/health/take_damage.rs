use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::DamageLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeDamage {
    pub level: DamageLevel,
    pub amount: NonZeroU8,
}

impl From<TakeDamage> for CharacterMutation {
    fn from(take_damage: TakeDamage) -> Self {
        CharacterMutation::TakeDamage(take_damage)
    }
}