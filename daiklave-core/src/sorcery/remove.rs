use crate::CharacterMutation;

/// A mutation to remove a circle of sorcery from a character.
pub struct RemoveSorcery;

impl From<RemoveSorcery> for CharacterMutation {
    fn from(_remove_sorcery: RemoveSorcery) -> Self {
        CharacterMutation::RemoveSorcery
    }
}
