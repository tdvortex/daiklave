use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::AbilityView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AbilityMemo {
    Zero,
    NonZero(u8, HashSet<String>),
}

impl<'source> AbilityMemo {
    pub fn as_ref(&'source self) -> AbilityView<'source> {
        match self {
            AbilityMemo::Zero => AbilityView::Zero,
            AbilityMemo::NonZero(dots, hashset) => {
                AbilityView::NonZero(*dots, hashset.iter().map(|s| s.as_str()).collect())
            }
        }
    }
}
