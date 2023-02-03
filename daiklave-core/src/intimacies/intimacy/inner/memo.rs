use serde::{Deserialize, Serialize};

use crate::intimacies::intimacy::{IntimacyLevel, IntimacyTypeMemo};

use super::IntimacyInner;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct IntimacyInnerMemo {
    pub intimacy_type: IntimacyTypeMemo,
    pub intimacy_level: IntimacyLevel,
    pub description: String,
}
