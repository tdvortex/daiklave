use serde::{Deserialize, Serialize};

use super::{night_ability::NightAbility};

/// An owned copy of Night Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NightMemo {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}