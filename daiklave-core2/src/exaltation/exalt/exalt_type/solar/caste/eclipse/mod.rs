mod ability;
mod anima_effect;
mod memo;

use std::collections::HashMap;

pub use ability::EclipseAbility;
pub(crate) use memo::EclipseMemo;

use crate::{abilities::AbilityName, charms::charm::EclipseCharm, exaltation::exalt::AnimaEffect};

use self::anima_effect::{ECLIPSE_ONE, ECLIPSE_THREE, ECLIPSE_TWO};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Eclipse<'source> {
    pub caste_not_supernal: [EclipseAbility; 4],
    pub supernal: EclipseAbility,
    pub eclipse_charms: HashMap<&'source str, &'source EclipseCharm>,
}

impl<'source> Eclipse<'source> {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
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

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn anima_effects(&self) -> [AnimaEffect<'static>; 3] {
        [ECLIPSE_ONE, ECLIPSE_TWO, ECLIPSE_THREE]
    }
}
