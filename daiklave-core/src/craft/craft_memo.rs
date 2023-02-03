use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityRatingMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct CraftMemo(pub(in crate::craft) HashMap<String, AbilityRatingMemo>);
