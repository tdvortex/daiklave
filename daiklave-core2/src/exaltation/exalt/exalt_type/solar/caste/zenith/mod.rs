mod ability;
mod anima_effect;
mod memo;

pub use ability::ZenithAbility;
pub(crate) use memo::ZenithMemo;

use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};

use self::anima_effect::{ZENITH_ONE, ZENITH_TWO, ZENITH_THREE};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Zenith {
    pub caste_not_supernal: [ZenithAbility; 4],
    pub supernal: ZenithAbility,
}

impl Zenith {
    pub(crate) fn as_memo(&self) -> ZenithMemo {
        ZenithMemo {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }

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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn anima_effects(&self) -> [AnimaEffect<'static>; 3] {
        [
            ZENITH_ONE,
            ZENITH_TWO,
            ZENITH_THREE
        ]
    }
}
