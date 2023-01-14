use serde::{Deserialize, Serialize};

use super::{supernal_ability::DawnSupernalLayout, Dawn};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DawnMemo {
    pub layout: DawnSupernalLayout,
}

impl<'source> DawnMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste) fn as_ref(&'source self) -> Dawn {
        Dawn {
            layout: self.layout,
        }
    }
}
