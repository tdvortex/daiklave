use serde::{Deserialize, Serialize};

use super::{ZenithAbility, ZenithView};

/// An owned copy of Zenith Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenithMemo {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}

impl<'source> ZenithMemo {
    pub fn as_ref(&'source self) -> ZenithView {
        ZenithView {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}