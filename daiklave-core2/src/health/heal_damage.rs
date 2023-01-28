use std::num::NonZeroU8;

use crate::CharacterMutation;

pub struct HealDamage(NonZeroU8);

impl HealDamage {
    pub fn new(amount: NonZeroU8) -> Self {
        Self(amount)
    }
}

impl From<HealDamage> for CharacterMutation {
    fn from(heal_damage: HealDamage) -> Self {
        CharacterMutation::HealDamage(heal_damage)
    }
}