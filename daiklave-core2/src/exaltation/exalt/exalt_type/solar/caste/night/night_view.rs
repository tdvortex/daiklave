use crate::abilities::AbilityName;

use super::{night_ability::NightAbility, builder::NightBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightView {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}

impl NightView {
    pub(crate) fn new (
        caste_not_supernal: [NightAbility; 4],
        supernal: NightAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub fn builder() -> NightBuilder {
        NightBuilder::default()
    }

    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
