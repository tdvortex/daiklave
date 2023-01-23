mod ability;
mod anima_effect;
mod memo;

pub use ability::NightAbility;
pub(crate) use memo::NightMemo;

use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};

use self::anima_effect::{NIGHT_ONE, NIGHT_TWO, NIGHT_THREE};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Night {
    pub caste_not_supernal: [NightAbility; 4],
    pub supernal: NightAbility,
}

impl Night {
    pub(crate) fn as_memo(&self) -> NightMemo {
        NightMemo::new(self.caste_not_supernal, self.supernal)
    }

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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn anima_effects(&self) -> [AnimaEffect<'static>; 3] {
        [
            NIGHT_ONE,
            NIGHT_TWO,
            NIGHT_THREE
        ]
    }
}
