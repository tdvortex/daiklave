use crate::CharacterMutation;

use super::FlawName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveFlaw(pub FlawName);

impl From<RemoveFlaw> for CharacterMutation {
    fn from(remove_flaw: RemoveFlaw) -> Self {
        CharacterMutation::RemoveFlaw(remove_flaw)
    }
}