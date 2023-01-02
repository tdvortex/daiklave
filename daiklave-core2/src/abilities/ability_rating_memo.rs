use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::AbilityRating;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AbilityRatingMemo {
    Zero,
    NonZero(u8, HashSet<String>),
}

impl<'source> AbilityRatingMemo {
    pub fn as_ref(&'source self) -> AbilityRating<'source> {
        match self {
            AbilityRatingMemo::Zero => AbilityRating::Zero,
            AbilityRatingMemo::NonZero(dots, hashset) => {
                AbilityRating::NonZero(*dots, hashset.iter().map(|s| s.as_str()).collect())
            }
        }
    }
}
