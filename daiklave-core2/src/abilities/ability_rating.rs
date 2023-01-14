use std::collections::HashSet;

use crate::CharacterMutationError;

use super::{AbilityError, AbilityRatingMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AbilityRating<'source> {
    Zero,
    NonZero(u8, HashSet<&'source str>),
}

impl<'source> Default for AbilityRating<'source> {
    fn default() -> Self {
        Self::Zero
    }
}

impl<'source> AbilityRating<'source> {
    pub fn as_memo(&self) -> AbilityRatingMemo {
        match self {
            AbilityRating::Zero => AbilityRatingMemo::Zero,
            AbilityRating::NonZero(dots, specialties) => AbilityRatingMemo::NonZero(
                *dots,
                specialties.iter().map(|s| s.to_string()).collect(),
            ),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            AbilityRating::Zero => 0,
            AbilityRating::NonZero(dots, _) => *dots,
        }
    }

    pub fn set_dots(&mut self, new_dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if new_dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else if new_dots == 0 {
            *self = AbilityRating::Zero;
            Ok(self)
        } else if let AbilityRating::NonZero(dots, _) = self {
            *dots = new_dots;
            Ok(self)
        } else {
            // Was zero, now is non zero
            *self = AbilityRating::NonZero(new_dots, HashSet::new());
            Ok(self)
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'source str> {
        let mut specialties = match self {
            AbilityRating::Zero => vec![],
            AbilityRating::NonZero(_, specialties) => specialties.iter().copied().collect(),
        };
        specialties.sort();

        specialties.into_iter()
    }

    pub fn add_specialty(
        &mut self,
        new_specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let AbilityRating::NonZero(_, specialties) = self {
            if specialties.contains(new_specialty) {
                Err(CharacterMutationError::AbilityError(
                    AbilityError::DuplicateSpecialty,
                ))
            } else {
                specialties.insert(new_specialty);
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::AbilityError(
                AbilityError::ZeroAbilitySpecialty,
            ))
        }
    }

    pub fn remove_specialty(
        &mut self,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let AbilityRating::NonZero(_, specialties) = self {
            if !specialties.remove(specialty) {
                Err(CharacterMutationError::AbilityError(
                    AbilityError::SpecialtyNotFound,
                ))
            } else {
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::AbilityError(
                AbilityError::SpecialtyNotFound,
            ))
        }
    }
}
