use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::artifact::wonders::{WonderId, WonderNoAttunementMemo};

use super::MortalWonders;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWondersMemo(pub(crate) HashMap<WonderId, WonderNoAttunementMemo>);

impl<'source> MortalWondersMemo {
    pub fn as_ref(&'source self) -> MortalWonders<'source> {
        MortalWonders(self.0.iter().map(|(k, v)| (*k, v.as_ref())).collect())
    }
}
