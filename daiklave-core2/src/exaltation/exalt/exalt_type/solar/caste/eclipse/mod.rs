mod builder;
mod eclipse_ability;
mod eclipse_memo;

pub(crate) use eclipse_ability::EclipseAbility;
pub use eclipse_memo::EclipseMemo;

use crate::abilities::AbilityName;

use self::builder::EclipseBuilder;

/// Caste traits for the Eclipse Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eclipse {
    caste_not_supernal: [EclipseAbility; 4],
    supernal: EclipseAbility,
}

impl Eclipse {
    pub(crate) fn new(caste_not_supernal: [EclipseAbility; 4], supernal: EclipseAbility) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    /// Builder struct for constructing Eclipse traits
    pub fn builder() -> EclipseBuilder {
        EclipseBuilder::default()
    }

    pub(crate) fn as_memo(&self) -> EclipseMemo {
        EclipseMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|eclipse_ability| AbilityName::from(*eclipse_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    /// Returns the Eclipse's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
