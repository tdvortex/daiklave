use serde::{Deserialize, Serialize};

use super::{Zenith, ZenithAbility};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ZenithMemo {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}

impl<'source> ZenithMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::zenith) fn new(
        caste_not_supernal: [ZenithAbility; 4],
        supernal: ZenithAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub(in crate::exaltation::exalt::exalt_type::solar::caste) fn as_ref(&'source self) -> Zenith {
        Zenith {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}
