use crate::abilities::AbilityName;

use super::{night_ability::NightAbility, night::Night};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightView {
    pub(crate) caste_not_supernal: [NightAbility; 4],
    pub(crate) supernal: NightAbility,
}

impl NightView {
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

    pub fn into_owned(self) -> Night {
        Night {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}