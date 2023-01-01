use crate::abilities::AbilityName;

use super::{builder::TwilightBuilder, twilight_ability::TwilightAbility, TwilightMemo};

/// Caste traits for the Twilight Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwilightView {
    caste_not_supernal: [TwilightAbility; 4],
    supernal: TwilightAbility,
}

impl TwilightView {
    pub(crate) fn new(caste_not_supernal: [TwilightAbility; 4], supernal: TwilightAbility) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    /// Builder struct for constructing Twilight traits
    pub fn builder() -> TwilightBuilder {
        TwilightBuilder::default()
    }

    pub(crate) fn as_memo(&self) -> TwilightMemo {
        TwilightMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|twilight_ability| AbilityName::from(*twilight_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    /// Returns the Twilight's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
