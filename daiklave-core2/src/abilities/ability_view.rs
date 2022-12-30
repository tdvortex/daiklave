use std::collections::HashSet;

use crate::CharacterMutationError;

use super::{AddSpecialtyError, RemoveSpecialtyError, SetAbilityError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AbilityView<'source> {
    Zero,
    NonZero(u8, HashSet<&'source str>),
}

impl<'source> Default for AbilityView<'source> {
    fn default() -> Self {
        Self::Zero
    }
}

impl<'source> AbilityView<'source> {
    pub fn dots(&self) -> u8 {
        match self {
            AbilityView::Zero => 0,
            AbilityView::NonZero(dots, _) => *dots,
        }
    }

    pub fn set_dots(&mut self, new_dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if new_dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(new_dots),
            ))
        } else if new_dots == 0 {
            *self = AbilityView::Zero;
            Ok(self)
        } else if let AbilityView::NonZero(dots, _) = self {
            *dots = new_dots;
            Ok(self)
        } else {
            // Was zero, now is non zero
            *self = AbilityView::NonZero(new_dots, HashSet::new());
            Ok(self)
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &str> {
        match self {
            AbilityView::Zero => vec![],
            AbilityView::NonZero(_, specialties) => specialties.iter().copied().collect(),
        }
        .into_iter()
    }

    pub fn add_specialty(
        &mut self,
        new_specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let AbilityView::NonZero(_, specialties) = self {
            if specialties.contains(new_specialty) {
                Err(CharacterMutationError::AddSpecialtyError(
                    AddSpecialtyError::DuplicateSpecialty,
                ))
            } else {
                specialties.insert(new_specialty);
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::AddSpecialtyError(
                AddSpecialtyError::ZeroAbility,
            ))
        }
    }

    pub fn remove_specialty(
        &mut self,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let AbilityView::NonZero(_, specialties) = self {
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
