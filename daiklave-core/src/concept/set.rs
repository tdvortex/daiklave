use crate::CharacterMutation;

/// A mutation to set the character's concept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetConcept(pub String);

impl From<SetConcept> for CharacterMutation {
    fn from(set_concept: SetConcept) -> Self {
        CharacterMutation::SetConcept(set_concept)
    }
}
