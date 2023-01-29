use crate::CharacterMutation;

use super::ability::AbilityNameQualifiedMutation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveSpecialty {
    pub ability_name: AbilityNameQualifiedMutation,
    pub specialty: String,
}

impl From<RemoveSpecialty> for CharacterMutation {
    fn from(remove_specialty: RemoveSpecialty) -> Self {
        CharacterMutation::RemoveSpecialty(remove_specialty)
    }
}