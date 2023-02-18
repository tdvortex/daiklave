use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityRatingMemo;

use super::Craft;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct CraftMemo(pub(in crate::craft) HashMap<String, AbilityRatingMemo>);

impl From<Craft<'_>> for CraftMemo {
    fn from(craft: Craft<'_>) -> Self {
        Self(craft.0.into_iter().map(|(focus, rating)| (focus.into(), (&rating).into())).collect())
    }
}