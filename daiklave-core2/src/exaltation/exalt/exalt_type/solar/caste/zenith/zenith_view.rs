use crate::abilities::AbilityName;

use super::{ZenithAbility, Zenith};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZenithView {
    pub(crate) caste_not_supernal: [ZenithAbility; 4],
    pub(crate) supernal: ZenithAbility,
}

impl ZenithView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn into_owned(self) -> Zenith {
        Zenith {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}