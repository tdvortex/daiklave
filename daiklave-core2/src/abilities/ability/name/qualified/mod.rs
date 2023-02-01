mod mutation;
pub(crate) use mutation::AbilityNameQualifiedMutation;

use crate::abilities::{AddSpecialty, RemoveSpecialty, SetAbility, AbilityError};

use super::AbilityNameVanilla;

/// A fully-qualified ability name, including the Craft focus or Martial Arts
/// style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityNameQualified<'source> {
    /// An ability with no additional qualifications needed.
    Vanilla(AbilityNameVanilla),
    /// A Craft ability, such as Craft (Weapons).
    Craft(&'source str),
    /// A Martial Arts style ability, such as Martial Arts (Crane Style).
    MartialArts(&'source str),
}

impl<'source> AbilityNameQualified<'source> {
    /// Creates a mutation to add a specialty to this ability.
    pub fn add_specialty(&self, specialty: impl Into<String>) -> AddSpecialty {
        AddSpecialty::new(*self, specialty)
    }

    /// Creates a mutation to set the dots to a specific level.
    pub fn set_dots(&self, dots: u8) -> Result<SetAbility, AbilityError> {
        SetAbility::new(*self, dots)
    }

    /// Creates a mutation to remove a specific specialty from this ability.
    pub fn remove_specialty(&self, specialty: impl Into<String>) -> RemoveSpecialty {
        RemoveSpecialty::new(*self, specialty)
    }
}

impl From<AbilityNameVanilla> for AbilityNameQualified<'_> {
    fn from(vanilla: AbilityNameVanilla) -> Self {
        Self::Vanilla(vanilla)
    }
}

impl<'source> From<&'source AbilityNameQualifiedMutation> for AbilityNameQualified<'source> {
    fn from(name: &'source AbilityNameQualifiedMutation) -> Self {
        match name {
            AbilityNameQualifiedMutation::Vanilla(vanilla) => (*vanilla).into(),
            AbilityNameQualifiedMutation::Craft(_) => todo!(),
            AbilityNameQualifiedMutation::MartialArts(_) => todo!(),
        }
    }
}