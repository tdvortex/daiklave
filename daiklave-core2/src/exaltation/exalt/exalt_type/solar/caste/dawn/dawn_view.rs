use crate::abilities::AbilityName;

use super::{
    dawn_memo::DawnMemo, dawn_caste_ability::DawnCasteAbility, dawn_supernal_ability::DawnSupernalAbility,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DawnView {
    pub(crate) caste_not_supernal: [DawnCasteAbility; 4],
    pub(crate) supernal: DawnSupernalAbility,
}

impl DawnView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn into_owned(self) -> DawnMemo {
        DawnMemo {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}
