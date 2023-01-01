use crate::abilities::AbilityName;

use super::{twilight_ability::TwilightAbility, builder::TwilightBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwilightView {
    pub(crate) caste_not_supernal: [TwilightAbility; 4],
    pub(crate) supernal: TwilightAbility,
}

impl TwilightView {
    pub(crate) fn new (
        caste_not_supernal: [TwilightAbility; 4],
        supernal: TwilightAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub fn builder() -> TwilightBuilder {
        TwilightBuilder::default()
    }

    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
