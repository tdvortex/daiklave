use std::{collections::HashSet, num::NonZeroU8};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AbilityRatingMemo {
    Zero,
    NonZero(NonZeroU8, HashSet<String>),
}