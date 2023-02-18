use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::artifact::wonders::WonderNoAttunementMemo;

use super::MortalWonders;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWondersMemo(pub(crate) HashMap<String, WonderNoAttunementMemo>);

impl From<&MortalWonders<'_>> for MortalWondersMemo {
    fn from(value: &MortalWonders<'_>) -> Self {
        Self(
            value
                .0
                .iter()
                .map(|(name, wonder)| ((*name).into(), wonder.into()))
                .collect(),
        )
    }
}
