use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::charms::charm::{SpiritCharmId, EclipseCharm};

use super::{Eclipse, EclipseAbility};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseMemo {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
    pub(crate) eclipse_charms: HashMap<SpiritCharmId, EclipseCharm>,
}

impl<'source> EclipseMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar::caste::eclipse) fn new(
        caste_not_supernal: [EclipseAbility; 4],
        supernal: EclipseAbility,
    ) -> Self {
        Self {
            caste_not_supernal,
            supernal,
            eclipse_charms: HashMap::new(),
        }
    }

    pub(in crate::exaltation::exalt::exalt_type::solar::caste) fn as_ref(&'source self) -> Eclipse {
        Eclipse {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}
