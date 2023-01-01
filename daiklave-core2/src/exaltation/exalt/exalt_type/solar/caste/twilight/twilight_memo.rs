use serde::{Deserialize, Serialize};
use super::{twilight_ability::TwilightAbility, TwilightView};

/// An owned copy of Twilight Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwilightMemo {
    caste_not_supernal: [TwilightAbility; 4],
    supernal: TwilightAbility,
}

impl<'source> TwilightMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::twilight) fn new(
        caste_not_supernal: [TwilightAbility; 4],
        supernal: TwilightAbility,
    ) -> Self {
        Self { caste_not_supernal, supernal }
    }

    pub fn as_ref(&'source self) -> TwilightView {
        TwilightView::new(self.caste_not_supernal, self.supernal)
    }
}