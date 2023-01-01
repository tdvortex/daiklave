use serde::{Deserialize, Serialize};

use super::{
    dawn_caste_ability::DawnCasteAbility,
    dawn_supernal_ability::DawnSupernalAbility,
};

/// An owned copy of Dawn Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DawnMemo {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}
