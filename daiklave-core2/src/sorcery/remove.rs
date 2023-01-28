use crate::CharacterMutation;

pub struct RemoveSorcery;

impl From<RemoveSorcery> for CharacterMutation {
    fn from(remove_sorcery: RemoveSorcery) -> Self {
        CharacterMutation::RemoveSorcery
    }
}