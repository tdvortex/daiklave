use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::{builder::NightBuilder, night_ability::NightAbility};

/// Caste traits for the Night Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Night {
    pub(crate) caste_not_supernal: [NightAbility; 4],
    pub(crate) supernal: NightAbility,
}

impl Night {
    /// Builder method
    pub fn builder() -> NightBuilder {
        NightBuilder::default()
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|night_ability| AbilityName::from(*night_ability) == ability)
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
