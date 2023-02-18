use std::{collections::HashSet, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use super::AbilityRating;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AbilityRatingMemo {
    Zero,
    NonZero(NonZeroU8, HashSet<String>),
}

impl From<&AbilityRating<'_>> for AbilityRatingMemo {
    fn from(rating: &AbilityRating<'_>) -> Self {
        match rating {
            AbilityRating::Zero => Self::Zero,
            AbilityRating::NonZero(dots, specialties) => {
                Self::NonZero(*dots, specialties.iter().map(|s| (*s).into()).collect())
            }
        }
    }
}
