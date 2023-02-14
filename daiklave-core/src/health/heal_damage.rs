use std::num::NonZeroU8;

use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

/// A mutation to heal a specified amount of damage from a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealDamage(pub NonZeroU8);

impl From<HealDamage> for CharacterMutation {
    fn from(heal_damage: HealDamage) -> Self {
        CharacterMutation::HealDamage(heal_damage)
    }
}
