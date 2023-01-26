use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::{mote_pool::MotePool, MoteCommitment};

use super::MotesState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MotesStateMemo {
    pub peripheral: MotePool,
    pub personal: MotePool,
    pub commitments: HashMap<String, MoteCommitment>,
}

impl<'source> MotesStateMemo {
    pub fn as_ref(&'source self) -> MotesState<'source> {
        MotesState {
            peripheral: self.peripheral,
            personal: self.personal,
            commitments: self
                .commitments
                .iter()
                .map(|(k, v)| (k.as_str(), *v))
                .collect(),
        }
    }
}
