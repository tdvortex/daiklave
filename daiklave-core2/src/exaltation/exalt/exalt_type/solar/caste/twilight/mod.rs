mod ability;
mod anima_effect;

pub use ability::TwilightAbility;

use serde::{Serialize, Deserialize};
use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};

use self::anima_effect::{TWILIGHT_ONE, TWILIGHT_THREE, TWILIGHT_TWO};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Twilight {
    pub caste_not_supernal: [TwilightAbility; 4],
    pub supernal: TwilightAbility,
}

impl Twilight {
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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn anima_effects(&self) -> [AnimaEffect<'static>; 3] {
        [TWILIGHT_ONE, TWILIGHT_TWO, TWILIGHT_THREE]
    }
}
