use crate::CharacterMutation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetName(pub String);

impl From<SetName> for CharacterMutation {
    fn from(set_name: SetName) -> Self {
        CharacterMutation::SetName(set_name)
    }
}