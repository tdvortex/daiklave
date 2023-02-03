use std::num::NonZeroU8;

use crate::CharacterMutation;

/// A mutation to recover some amount of spent motes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoverMotes(pub NonZeroU8);

impl From<RecoverMotes> for CharacterMutation {
    fn from(recover_motes: RecoverMotes) -> Self {
        CharacterMutation::RecoverMotes(recover_motes)
    }
}
