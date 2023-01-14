use serde::{Deserialize, Serialize};

use super::{
    caste_ability::DawnCasteAbility, supernal_ability::DawnSupernalAbility, Dawn,
};

/// An owned copy of Dawn Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DawnMemo {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

impl<'source> DawnMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::dawn) fn new(
        caste_not_supernal: [DawnCasteAbility; 4],
        supernal: DawnSupernalAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub(in crate::exaltation::exalt::exalt_type::solar::caste) fn as_ref(&'source self) -> Dawn {
        Dawn {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}
