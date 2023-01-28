use crate::CharacterMutation;

use super::ability::AbilityNameQualifiedMutation;

pub struct AddSpecialty {
    ability_name: AbilityNameQualifiedMutation,
    specialty: String,
}

impl AddSpecialty {
    pub fn new(ability_name: AbilityNameQualifiedMutation, specialty: String) -> Self {
        Self {
            ability_name,
            specialty,
        }
    }
}

impl From<AddSpecialty> for CharacterMutation {
    fn from(add_specialty: AddSpecialty) -> Self {
        CharacterMutation::AddSpecialty(add_specialty)
    }
}