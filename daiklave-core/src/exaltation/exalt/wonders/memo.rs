use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::artifact::wonders::WonderNoAttunementMemo;

use super::ExaltWonders;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWondersMemo(
    pub(crate) HashMap<String, (WonderNoAttunementMemo, Option<u8>)>,
);

impl From<&ExaltWonders<'_>> for ExaltWondersMemo {
    fn from(value: &ExaltWonders<'_>) -> Self {
        Self(
            value
                .0
                .iter()
                .map(|(name, (wonder, attunement))| ((*name).into(), (wonder.into(), *attunement)))
                .collect(),
        )
    }
}
