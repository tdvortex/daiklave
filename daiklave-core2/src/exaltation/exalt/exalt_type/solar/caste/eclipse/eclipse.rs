use serde::{Serialize, Deserialize};

use crate::abilities::AbilityName;

use super::{EclipseAbility, builder::EclipseBuilder};

/// Caste traits for the Eclipse Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eclipse {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
}

impl Eclipse {
    /// Builder method
    pub fn builder() -> EclipseBuilder {
        EclipseBuilder::default()
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
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

    pub(crate) fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}