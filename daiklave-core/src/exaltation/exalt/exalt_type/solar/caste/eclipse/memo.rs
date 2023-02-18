use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::charms::charm::{spirit::SpiritCharmName, EclipseCharm};

use super::{Eclipse, EclipseAbility};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseMemo {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
    pub(crate) eclipse_charms: HashMap<SpiritCharmName, EclipseCharm>,
}

impl From<&Eclipse<'_>> for EclipseMemo {
    fn from(value: &Eclipse<'_>) -> Self {
        Self {
            caste_not_supernal: value.caste_not_supernal,
            supernal: value.supernal,
            eclipse_charms: value
                .eclipse_charms
                .iter()
                .map(|(name, charm)| ((*name).into(), (*charm).to_owned()))
                .collect(),
        }
    }
}
