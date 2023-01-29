mod ability;
mod anima_effect;

pub use ability::NightAbility;

use serde::{Serialize, Deserialize};
use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};

use self::anima_effect::{NIGHT_ONE, NIGHT_THREE, NIGHT_TWO};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Night {
    pub caste_not_supernal: [NightAbility; 4],
    pub supernal: NightAbility,
}

impl Night {
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
        [NIGHT_ONE, NIGHT_TWO, NIGHT_THREE]
    }
}
