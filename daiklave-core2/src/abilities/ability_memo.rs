use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::Ability;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AbilityMemo {
    Zero,
    NonZero(u8, HashSet<String>),
}

impl<'source> AbilityMemo {
    pub fn as_ref(&'source self) -> Ability<'source> {
        match self {
            AbilityMemo::Zero => Ability::Zero,
            AbilityMemo::NonZero(dots, hashset) => {
                Ability::NonZero(*dots, hashset.iter().map(|s| s.as_str()).collect())
            }
        }
    }
}
