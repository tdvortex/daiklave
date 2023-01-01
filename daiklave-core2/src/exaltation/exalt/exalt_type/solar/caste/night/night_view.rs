use crate::abilities::AbilityName;

use super::{builder::NightBuilder, night_ability::NightAbility, NightMemo};

/// Caste traits for the Night Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightView {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}

impl NightView {
    pub(crate) fn new(caste_not_supernal: [NightAbility; 4], supernal: NightAbility) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    /// Builder struct for constructing Night traits
    pub fn builder() -> NightBuilder {
        NightBuilder::default()
    }

    pub(crate) fn as_memo(&self) -> NightMemo {
        NightMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|night_ability| AbilityName::from(*night_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    /// Returns the Night's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
