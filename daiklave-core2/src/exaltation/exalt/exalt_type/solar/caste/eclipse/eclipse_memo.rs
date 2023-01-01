use serde::{Deserialize, Serialize};

use super::{EclipseAbility};

/// An owned copy of Eclipse Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseMemo {
    caste_not_supernal: [EclipseAbility; 4],
    supernal: EclipseAbility,
}
