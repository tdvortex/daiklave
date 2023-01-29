use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::motes::MotesStateMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceStateMemo {
    pub rating: NonZeroU8,
    pub motes: MotesStateMemo,
}