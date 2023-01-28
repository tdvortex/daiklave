use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::artifact::wonders::WonderNoAttunementMemo;

use super::ExaltWonders;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWondersMemo(
    pub(crate) HashMap<String, (WonderNoAttunementMemo, Option<u8>)>,
);

impl<'source> ExaltWondersMemo {
    pub fn as_ref(&'source self) -> ExaltWonders<'source> {
        ExaltWonders(
            self.0
                .iter()
                .map(|(k, (no_attunement, attunement))| {
                    (k.as_str(), (no_attunement.as_ref(), *attunement))
                })
                .collect(),
        )
    }
}
