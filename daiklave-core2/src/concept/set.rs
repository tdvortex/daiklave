use crate::CharacterMutation;

pub struct SetConcept(String);

impl SetConcept {
    pub fn new(concept: String) -> Self {
        Self(concept)
    }
}

impl From<SetConcept> for CharacterMutation {
    fn from(set_concept: SetConcept) -> Self {
        CharacterMutation::SetConcept(set_concept)
    }
}