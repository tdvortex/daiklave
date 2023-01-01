use serde::{Deserialize, Serialize};

use super::{night_ability::NightAbility, NightView};

/// An owned copy of Night Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NightMemo {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}

impl<'source> NightMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::night) fn new (
        caste_not_supernal: [NightAbility; 4],
        supernal: NightAbility,        
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub fn as_ref(&'source self) -> NightView {
        NightView::new(
            self.caste_not_supernal,
            self.supernal,
        )
    }
}