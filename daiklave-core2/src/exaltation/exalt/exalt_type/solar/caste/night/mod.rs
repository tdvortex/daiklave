mod ability;
mod memo;

pub use ability::NightAbility;
pub(crate) use memo::NightMemo;

use crate::abilities::AbilityName;

/// Caste traits for the Night Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Night {
    pub caste_not_supernal: [NightAbility; 4],
    pub supernal: NightAbility,
}

impl Night {
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
