use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::charms::charm::{spirit::SpiritCharmName, EclipseCharm};

use super::EclipseAbility;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseMemo {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
    pub(crate) eclipse_charms: HashMap<SpiritCharmName, EclipseCharm>,
}
