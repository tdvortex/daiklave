use crate::CharacterMutation;

pub struct SetName(String);

impl SetName {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl From<SetName> for CharacterMutation {
    fn from(set_name: SetName) -> Self {
        CharacterMutation::SetName(set_name)
    }
}