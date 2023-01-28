use std::num::NonZeroU8;

use crate::CharacterMutation;

pub struct RecoverMotes(NonZeroU8);

impl RecoverMotes {
    pub fn new(amount: NonZeroU8) -> Self {
        Self(amount)
    }
}

impl From<RecoverMotes> for CharacterMutation {
    fn from(recover_motes: RecoverMotes) -> Self {
        CharacterMutation::RecoverMotes(recover_motes)
    }
}