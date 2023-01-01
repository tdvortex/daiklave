use serde::{Deserialize, Serialize};

use super::{motes_memo::MotesMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceMemo {
    rating: u8,
    motes: MotesMemo,
}
