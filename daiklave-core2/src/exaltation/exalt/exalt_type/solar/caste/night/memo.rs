use serde::{Deserialize, Serialize};

use super::{ability::NightAbility, Night};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NightMemo {
    pub(crate) caste_not_supernal: [NightAbility; 4],
    pub(crate) supernal: NightAbility,
}

impl<'source> NightMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::night) fn new(
        caste_not_supernal: [NightAbility; 4],
        supernal: NightAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub(in crate::exaltation::exalt::exalt_type::solar::caste) fn as_ref(&'source self) -> Night {
        Night {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}
