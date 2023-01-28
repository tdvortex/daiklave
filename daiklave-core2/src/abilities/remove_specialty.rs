use crate::CharacterMutation;

use super::ability::AbilityNameQualifiedMutation;

pub struct RemoveSpecialty {
    ability_name: AbilityNameQualifiedMutation,
    specialty: String,
}

impl RemoveSpecialty {
    pub fn new(ability_name: AbilityNameQualifiedMutation, specialty: String) -> Self {
        Self {
            ability_name,
            specialty,
        }
    }
}

impl From<RemoveSpecialty> for CharacterMutation {
    fn from(remove_specialty: RemoveSpecialty) -> Self {
        CharacterMutation::AddSpecialty(remove_specialty)
    }
}