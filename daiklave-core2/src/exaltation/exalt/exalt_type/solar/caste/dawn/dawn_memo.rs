use serde::{Deserialize, Serialize};

use super::{
    dawn_caste_ability::DawnCasteAbility,
    dawn_supernal_ability::DawnSupernalAbility, DawnView,
};

/// An owned copy of Dawn Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DawnMemo {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

impl<'source> DawnMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::dawn) fn new (
        caste_not_supernal: [DawnCasteAbility; 4],
        supernal: DawnSupernalAbility,
    ) -> Self {
        Self { caste_not_supernal, supernal }
    }

    pub fn as_ref(&'source self) -> DawnView {
        DawnView::new(
            self.caste_not_supernal,
            self.supernal,
        )
    }
}
