use crate::CharacterMutation;

use super::ability::AbilityNameQualifiedMutation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSpecialty {
    pub ability_name: AbilityNameQualifiedMutation,
    pub specialty: String,
}

impl From<AddSpecialty> for CharacterMutation {
    fn from(add_specialty: AddSpecialty) -> Self {
        CharacterMutation::AddSpecialty(add_specialty)
    }
}