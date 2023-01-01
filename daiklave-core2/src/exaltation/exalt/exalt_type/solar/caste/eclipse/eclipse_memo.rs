use serde::{Deserialize, Serialize};

use super::{EclipseAbility, EclipseView};

/// An owned copy of Eclipse Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseMemo {
    caste_not_supernal: [EclipseAbility; 4],
    supernal: EclipseAbility,
}

impl<'source> EclipseMemo {
    pub fn as_ref(&'source self) -> EclipseView {
        EclipseView {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}