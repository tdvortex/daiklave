use crate::CharacterMutation;

/// A mutation to set the character to be mortal, removing any prior
/// Exaltation.
pub struct SetMortal;

impl From<SetMortal> for CharacterMutation {
    fn from(_set_mortal: SetMortal) -> Self {
        CharacterMutation::SetMortal
    }
}