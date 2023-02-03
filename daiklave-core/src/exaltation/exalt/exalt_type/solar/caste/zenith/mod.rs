mod ability;
mod anima_effect;

pub use ability::ZenithAbility;

use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};
use serde::{Deserialize, Serialize};

use self::anima_effect::{ZENITH_ONE, ZENITH_THREE, ZENITH_TWO};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Zenith {
    pub caste_not_supernal: [ZenithAbility; 4],
    pub supernal: ZenithAbility,
}

impl Zenith {
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
        [ZENITH_ONE, ZENITH_TWO, ZENITH_THREE]
    }
}
