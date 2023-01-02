mod builder;
mod zenith_ability;
mod zenith_memo;

pub(crate) use zenith_ability::ZenithAbility;
pub use zenith_memo::ZenithMemo;

use crate::abilities::AbilityName;

use self::builder::ZenithBuilder;

/// Caste traits for the Zenith Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zenith {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}

impl Zenith {
    pub(crate) fn new(caste_not_supernal: [ZenithAbility; 4], supernal: ZenithAbility) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    /// Builder struct for constructing Zenith traits
    pub fn builder() -> ZenithBuilder {
        ZenithBuilder::default()
    }

    pub(crate) fn as_memo(&self) -> ZenithMemo {
        ZenithMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability.
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

    /// Returns the Zenith's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
