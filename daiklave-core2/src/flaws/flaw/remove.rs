use crate::CharacterMutation;

use super::FlawName;

pub struct RemoveFlaw(FlawName);

impl RemoveFlaw {
    pub fn new(flaw_name: FlawName) -> Self {
        Self(flaw_name)
    }
}

impl From<RemoveFlaw> for CharacterMutation {
    fn from(remove_flaw: RemoveFlaw) -> Self {
        CharacterMutation::RemoveFlaw(remove_flaw)
    }
}