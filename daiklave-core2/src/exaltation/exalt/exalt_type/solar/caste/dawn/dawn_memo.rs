use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::{
    builder::DawnBuilder, dawn_caste_ability::DawnCasteAbility,
    dawn_supernal_ability::DawnSupernalAbility,
};

/// Caste traits for the Dawn Caste Solar. Note that because of
/// Brawl/MartialArts, Dawns have 5 possible Caste abilities but 6 possible
/// Supernal abilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DawnMemo {
    pub(crate) caste_not_supernal: [DawnCasteAbility; 4],
    pub(crate) supernal: DawnSupernalAbility,
}

impl DawnMemo {
    /// Builder method
    pub fn builder() -> DawnBuilder {
        DawnBuilder::default()
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|dawn_caste_ability| AbilityName::from(*dawn_caste_ability) == ability)
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
