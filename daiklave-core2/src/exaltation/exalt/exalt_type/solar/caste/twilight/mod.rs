mod ability;
mod memo;

pub use ability::TwilightAbility;
pub(crate) use memo::TwilightMemo;

use crate::abilities::AbilityName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Twilight {
    pub caste_not_supernal: [TwilightAbility; 4],
    pub supernal: TwilightAbility,
}

impl Twilight {
    pub(crate) fn as_memo(&self) -> TwilightMemo {
        TwilightMemo {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
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
