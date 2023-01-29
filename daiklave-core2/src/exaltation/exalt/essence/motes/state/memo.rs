use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::{mote_pool::MotePool, MoteCommitment};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MotesStateMemo {
    pub peripheral: MotePool,
    pub personal: MotePool,
    pub commitments: HashMap<String, MoteCommitment>,
}