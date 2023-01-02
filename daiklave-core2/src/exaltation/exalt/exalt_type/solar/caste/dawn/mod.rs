mod builder;
mod dawn_caste_ability;
mod dawn_memo;
mod dawn_supernal_ability;

pub(crate) use dawn_caste_ability::DawnCasteAbility;
pub use dawn_memo::DawnMemo;
pub(crate) use dawn_supernal_ability::DawnSupernalAbility;

use crate::abilities::AbilityName;

use self::builder::DawnBuilder;

/// Caste traits for the Dawn Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dawn {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

impl Dawn {
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
