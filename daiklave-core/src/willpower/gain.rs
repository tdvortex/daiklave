use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::CharacterMutation;

/// A mutation to increase the character's current available willpower. This
/// may take them above their permanent rating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GainWillpower(pub NonZeroU8);

impl From<GainWillpower> for CharacterMutation {
    fn from(gain_willpower: GainWillpower) -> Self {
        CharacterMutation::GainWillpower(gain_willpower)
    }
}
