use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityMemo;

use super::Craft;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct CraftMemo(pub(in crate::craft) HashMap<String, AbilityMemo>);

impl<'source> CraftMemo {
    pub fn as_ref(&'source self) -> Craft<'source> {
        Craft(
            self.0
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect(),
        )
    }
}
