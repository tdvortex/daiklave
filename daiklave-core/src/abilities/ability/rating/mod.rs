mod memo;
pub(crate) use memo::AbilityRatingMemo;

use std::{collections::HashSet, num::NonZeroU8};

use crate::{abilities::AbilityError, character::mutation::CharacterMutationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AbilityRating<'source> {
    Zero,
    NonZero(NonZeroU8, HashSet<&'source str>),
}

impl<'source> Default for AbilityRating<'source> {
    fn default() -> Self {
        Self::Zero
    }
}

impl<'source> From<&'source AbilityRatingMemo> for AbilityRating<'source> {
    fn from(value: &'source AbilityRatingMemo) -> Self {
        match value {
            AbilityRatingMemo::Zero => Self::Zero,
            AbilityRatingMemo::NonZero(dots, specialties) => {
                Self::NonZero(*dots, specialties.iter().map(|s| s.as_str()).collect())
            }
        }
    }
}

impl<'source> AbilityRating<'source> {
    pub fn dots(&self) -> u8 {
        match self {
            AbilityRating::Zero => 0,
            AbilityRating::NonZero(dots, _) => dots.get(),
        }
    }

    pub fn set_dots(&mut self, new_dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if new_dots > 5 {
            return Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ));
        }

        if let Some(nonzero) = NonZeroU8::new(new_dots) {
            if let AbilityRating::NonZero(dots, _) = self {
                *dots = nonzero;
            } else {
                // Was zero, now is non zero
                *self = AbilityRating::NonZero(nonzero, HashSet::new());
            }
        } else {
            *self = AbilityRating::Zero;
        }
        Ok(self)
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
            if !specialties.insert(new_specialty) {
                Err(CharacterMutationError::AbilityError(
                    AbilityError::DuplicateSpecialty,
                ))
            } else {
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
