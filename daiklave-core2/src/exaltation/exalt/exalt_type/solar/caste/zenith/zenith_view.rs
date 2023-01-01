use crate::abilities::AbilityName;

use super::{ZenithAbility, builder::ZenithBuilder, ZenithMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZenithView {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}

impl ZenithView {
    pub(crate) fn new (
        caste_not_supernal: [ZenithAbility; 4],
        supernal: ZenithAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub fn builder() -> ZenithBuilder {
        ZenithBuilder::default()
    }

    pub(crate) fn as_memo(&self) -> ZenithMemo {
        ZenithMemo::new(
            self.caste_not_supernal,
            self.supernal,
        )
    }

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
}
