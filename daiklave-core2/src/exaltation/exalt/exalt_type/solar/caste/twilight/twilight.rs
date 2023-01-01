use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::{builder::TwilightBuilder, twilight_ability::TwilightAbility};

/// Caste traits for the Twilight Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Twilight {
    pub(crate) caste_not_supernal: [TwilightAbility; 4],
    pub(crate) supernal: TwilightAbility,
}

impl Twilight {
    /// Builder method
    pub fn builder() -> TwilightBuilder {
        TwilightBuilder::default()
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|twilight_ability| AbilityName::from(*twilight_ability) == ability)
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
