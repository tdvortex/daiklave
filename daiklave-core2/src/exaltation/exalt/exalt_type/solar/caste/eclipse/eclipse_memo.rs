use serde::{Deserialize, Serialize};

use super::{EclipseAbility, EclipseView};

/// An owned copy of Eclipse Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseMemo {
    caste_not_supernal: [EclipseAbility; 4],
    supernal: EclipseAbility,
}

impl<'source> EclipseMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::eclipse) fn new(
        caste_not_supernal: [EclipseAbility; 4],
        supernal: EclipseAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
        }
    }

    pub fn as_ref(&'source self) -> EclipseView {
        EclipseView::new(
            self.caste_not_supernal,
            self.supernal,
        )
    }
}