use crate::CharacterMutation;

pub struct SetMortal;

impl From<SetMortal> for CharacterMutation {
    fn from(set_mortal: SetMortal) -> Self {
        CharacterMutation::SetMortal
    }
}