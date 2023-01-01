use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::CharacterMutationError;

use super::{SetAbilityError, AddSpecialtyError, RemoveSpecialtyError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Ability {
    Zero,
    NonZero(u8, HashSet<String>),
}

impl Default for Ability {
    fn default() -> Self {
        Self::Zero
    }
}

impl Ability {
    pub fn dots(&self) -> u8 {
        match self {
            Ability::Zero => 0,
            Ability::NonZero(dots, _) => *dots,
        }
    }

    pub fn set_dots(&mut self, new_dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if new_dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(new_dots),
            ))
        } else if new_dots == 0 {
            *self = Ability::Zero;
            Ok(self)
        } else if let Ability::NonZero(dots, _) = self {
            *dots = new_dots;
            Ok(self)
        } else {
            // Was zero, now is non zero
            *self = Ability::NonZero(new_dots, HashSet::new());
            Ok(self)
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &str> {
        match self {
            Ability::Zero => vec![],
            Ability::NonZero(_, specialties) => specialties.iter().map(|s| s.as_str()).collect(),
        }
        .into_iter()
    }

    pub fn add_specialty(&mut self, new_specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        if let Ability::NonZero(_, specialties) = self {
            if specialties.contains(new_specialty) {
                Err(CharacterMutationError::AddSpecialtyError(
                    AddSpecialtyError::DuplicateSpecialty,
                ))
            } else {
                specialties.insert(new_specialty.to_owned());
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::AddSpecialtyError(
                AddSpecialtyError::ZeroAbility,
            ))
        }
    }

    pub fn remove_specialty(&mut self, specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        if let Ability::NonZero(_, specialties) = self {
            if !specialties.remove(specialty) {
                Err(CharacterMutationError::RemoveSpecialtyError(
                    RemoveSpecialtyError::NotFound,
                ))
            } else {
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::RemoveSpecialtyError(
                RemoveSpecialtyError::NotFound,
            ))
        }
    }
}