use std::num::NonZeroU8;

use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::DamageLevel;

/// A mutation to add damage to a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TakeDamage {
    /// The level of damage applied.
    pub level: DamageLevel,
    /// The amount of damage to add.
    pub amount: NonZeroU8,
}

impl From<TakeDamage> for CharacterMutation {
    fn from(take_damage: TakeDamage) -> Self {
        CharacterMutation::TakeDamage(take_damage)
    }
}
