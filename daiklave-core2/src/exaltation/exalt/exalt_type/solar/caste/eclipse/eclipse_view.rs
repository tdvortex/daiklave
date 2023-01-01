use crate::abilities::AbilityName;

use super::{EclipseAbility, Eclipse};

/// Caste traits for the Eclipse Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EclipseView {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
}

impl EclipseView {
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

    pub(crate) fn into_owned(self) -> Eclipse {
        Eclipse {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}