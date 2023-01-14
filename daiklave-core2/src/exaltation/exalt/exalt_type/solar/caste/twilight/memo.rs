use super::{ability::TwilightAbility, Twilight};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TwilightMemo {
    pub caste_not_supernal: [TwilightAbility; 4],
    pub supernal: TwilightAbility,
}

impl<'source> TwilightMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste) fn as_ref(
        &'source self,
    ) -> Twilight {
        Twilight {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}
