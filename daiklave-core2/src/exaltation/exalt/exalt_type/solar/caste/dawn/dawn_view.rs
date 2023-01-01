use crate::abilities::AbilityName;

use super::{
    dawn_caste_ability::DawnCasteAbility, dawn_supernal_ability::DawnSupernalAbility, builder::DawnBuilder,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DawnView {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

impl DawnView {
    pub(crate) fn new (
        caste_not_supernal: [DawnCasteAbility; 4],
        supernal: DawnSupernalAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub fn builder() -> DawnBuilder {
        DawnBuilder::default()
    }

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
}
