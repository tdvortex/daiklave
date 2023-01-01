use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::{builder::ZenithBuilder, ZenithAbility};

/// Caste traits for the Zenith Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenithMemo {
    pub(crate) caste_not_supernal: [ZenithAbility; 4],
    pub(crate) supernal: ZenithAbility,
}

impl ZenithMemo {
    /// Builder method
    pub fn builder() -> ZenithBuilder {
        ZenithBuilder::default()
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|zenith_caste_ability| AbilityName::from(*zenith_caste_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub(crate) fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
