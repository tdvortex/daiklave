use serde::{Deserialize, Serialize};

use super::{ZenithAbility};

/// An owned copy of Zenith Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenithMemo {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}