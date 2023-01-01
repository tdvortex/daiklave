use crate::abilities::AbilityName;

use super::{
    builder::DawnBuilder, dawn_caste_ability::DawnCasteAbility,
    dawn_supernal_ability::DawnSupernalAbility, DawnMemo,
};

/// Caste traits for the Dawn Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DawnView {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

impl DawnView {
    pub(crate) fn new(
        caste_not_supernal: [DawnCasteAbility; 4],
        supernal: DawnSupernalAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    /// Builder struct for constructing Dawn traits
    pub fn builder() -> DawnBuilder {
        DawnBuilder::default()
    }

    pub(crate) fn as_memo(&self) -> DawnMemo {
        DawnMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability.
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

    /// Returns the Dawn's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
