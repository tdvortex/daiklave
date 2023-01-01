use serde::{Deserialize, Serialize};

use super::{night_ability::NightAbility, NightView};

/// An owned copy of Night Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NightMemo {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}

impl<'source> NightMemo {
    pub fn as_ref(&'source self) -> NightView {
        NightView {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}