use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::artifact::wonders::WonderNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWondersMemo(
    pub(crate) HashMap<String, (WonderNoAttunementMemo, Option<u8>)>,
);
