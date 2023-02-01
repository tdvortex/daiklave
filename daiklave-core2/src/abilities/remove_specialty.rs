use crate::CharacterMutation;

use super::{ability::AbilityNameQualifiedMutation, AbilityNameQualified};

/// A mutation to remove a specialty from a specific ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveSpecialty {
    pub(crate) ability_name: AbilityNameQualifiedMutation,
    pub(crate) specialty: String,
}

impl RemoveSpecialty {
    /// Creates a new RemoveSpecialty mutation.
    pub fn new(ability_name: AbilityNameQualified<'_>, specialty: impl Into<String>) -> Self {
        Self {
            ability_name: ability_name.into(),
            specialty: specialty.into(),
        }
    }
}

impl From<RemoveSpecialty> for CharacterMutation {
    fn from(remove_specialty: RemoveSpecialty) -> Self {
        CharacterMutation::RemoveSpecialty(remove_specialty)
    }
}