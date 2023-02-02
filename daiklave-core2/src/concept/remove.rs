use crate::CharacterMutation;

/// A mutation to remove the character's concept.
pub struct RemoveConcept;

impl From<RemoveConcept> for CharacterMutation {
    fn from(_remove_concept: RemoveConcept) -> Self {
        CharacterMutation::RemoveConcept
    }
}
