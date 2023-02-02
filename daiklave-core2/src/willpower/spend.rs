use std::num::NonZeroU8;

use crate::CharacterMutation;

/// A mutation to spend the character's willpower.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendWillpower(pub NonZeroU8);

impl From<SpendWillpower> for CharacterMutation {
    fn from(spend_willpower: SpendWillpower) -> Self {
        CharacterMutation::SpendWillpower(spend_willpower)
    }
}
