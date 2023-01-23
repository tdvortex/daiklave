mod ability;
mod memo;

use std::collections::HashMap;

pub use ability::EclipseAbility;
pub(crate) use memo::EclipseMemo;

use crate::{
    abilities::AbilityName,
    charms::charm::{EclipseCharm, SpiritCharmId},
};

/// Caste traits for the Eclipse Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Eclipse<'source> {
    pub caste_not_supernal: [EclipseAbility; 4],
    pub supernal: EclipseAbility,
    pub eclipse_charms: HashMap<SpiritCharmId, &'source EclipseCharm>,
}

impl<'source> Eclipse<'source> {
    pub(crate) fn as_memo(&self) -> EclipseMemo {
        EclipseMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability.
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

    /// Returns the Eclipse's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
