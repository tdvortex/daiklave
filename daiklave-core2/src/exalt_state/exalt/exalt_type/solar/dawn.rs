use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

mod builder;
mod dawn_caste_ability;
mod dawn_supernal_ability;
mod dawn_view;
pub use builder::DawnBuilder;
pub(crate) use dawn_caste_ability::DawnCasteAbility;
pub(crate) use dawn_supernal_ability::DawnSupernalAbility;
pub(crate) use dawn_view::DawnView;

/// Caste traits for the Dawn Caste Solar. Note that because of
/// Brawl/MartialArts, Dawns have 5 possible Caste abilities but 6 possible
/// Supernal abilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dawn {
    pub(crate) caste_not_supernal: [DawnCasteAbility; 4],
    pub(crate) supernal: DawnSupernalAbility,
}

impl Dawn {
    /// Builder method
    pub fn builder() -> DawnBuilder {
        DawnBuilder {
            caste_not_supernal: HashSet::new(),
            supernal: None,
        }
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|dawn_caste_ability| AbilityName::from(*dawn_caste_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub(crate) fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}