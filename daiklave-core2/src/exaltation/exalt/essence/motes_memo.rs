use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{mote_pool::MotePool, motes::Motes, MoteCommitmentId, MoteCommitmentMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MotesMemo {
    peripheral: MotePool,
    personal: MotePool,
    commitments: HashMap<MoteCommitmentId, MoteCommitmentMemo>,
}

impl<'source> MotesMemo {
    pub(in crate::exaltation::exalt::essence) fn new(
        peripheral: MotePool,
        personal: MotePool,
        commitments: HashMap<MoteCommitmentId, MoteCommitmentMemo>,
    ) -> Self {
        Self {
            peripheral,
            personal,
            commitments,
        }
    }

    pub fn as_ref(&'source self) -> Motes<'source> {
        Motes::new(
            self.peripheral,
            self.personal,
            self.commitments
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        )
    }
}
