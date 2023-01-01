use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityMemo};

use super::CraftView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct CraftMemo(HashMap<String, AbilityMemo>);

impl<'source> CraftMemo {
    pub fn as_ref(&'source self) -> CraftView<'source> {
        CraftView(self.0.iter().map(|(k, v)| (k.as_str(), v.as_ref())).collect())
    }
}