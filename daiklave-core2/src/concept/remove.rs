use crate::CharacterMutation;

pub struct RemoveConcept;

impl From<RemoveConcept> for CharacterMutation {
    fn from(_remove_concept: RemoveConcept) -> Self {
        CharacterMutation::RemoveConcept
    }
}
