use serde::{Deserialize, Serialize};
use super::{twilight_ability::TwilightAbility, TwilightView};

/// An owned copy of Twilight Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwilightMemo {
    caste_not_supernal: [TwilightAbility; 4],
    supernal: TwilightAbility,
}

impl<'source> TwilightMemo {
    pub fn as_ref(&'source self) -> TwilightView {
        TwilightView { caste_not_supernal: self.caste_not_supernal, supernal: self.supernal }
    }
}