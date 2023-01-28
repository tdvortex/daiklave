use std::num::NonZeroU8;

use crate::CharacterMutation;

pub struct SpendWillpower(NonZeroU8);

impl SpendWillpower {
    pub fn new(amount: NonZeroU8) -> Self {
        Self(amount)
    }
}

impl From<SpendWillpower> for CharacterMutation {
    fn from(spend_willpower: SpendWillpower) -> Self {
        CharacterMutation::SpendWillpower(spend_willpower)
    }
}