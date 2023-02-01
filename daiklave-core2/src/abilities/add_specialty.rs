use crate::CharacterMutation;

use super::{ability::AbilityNameQualifiedMutation, AbilityNameQualified};

/// A mutation to add a specialty to an ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSpecialty {
    pub(crate) ability_name: AbilityNameQualifiedMutation,
    pub(crate) specialty: String,
}

impl AddSpecialty {
    /// Creates a new AddSpecialty mutation.
    pub fn new(ability_name: AbilityNameQualified<'_>, specialty: impl Into<String>) -> Self {
        Self {
            ability_name: ability_name.into(),
            specialty: specialty.into(),
        }
    }
}


impl From<AddSpecialty> for CharacterMutation {
    fn from(add_specialty: AddSpecialty) -> Self {
        CharacterMutation::AddSpecialty(add_specialty)
    }
}