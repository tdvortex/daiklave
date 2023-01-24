use serde::{Deserialize, Serialize};

use crate::intimacies::intimacy::{IntimacyTypeMemo, IntimacyLevel};

use super::IntimacyInner;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct IntimacyInnerMemo {
    pub intimacy_type: IntimacyTypeMemo,
    pub intimacy_level: IntimacyLevel,
    pub description: String,
}

impl<'source> IntimacyInnerMemo {
    pub fn as_ref(&'source self) -> IntimacyInner<'source> {
        IntimacyInner {
            intimacy_type: self.intimacy_type.as_ref(),
            intimacy_level: self.intimacy_level,
            description: self.description.as_str(),
        }
    }
}