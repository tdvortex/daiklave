use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::{
    mote_commitment::MoteCommitmentMemo, mote_pool::MotePool, OtherMoteCommitmentId,
};

use super::MotesState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MotesStateMemo {
    peripheral: MotePool,
    personal: MotePool,
    commitments: HashMap<OtherMoteCommitmentId, MoteCommitmentMemo>,
}

impl<'source> MotesStateMemo {
    pub(in crate::exaltation::exalt::essence) fn new(
        peripheral: MotePool,
        personal: MotePool,
        commitments: HashMap<OtherMoteCommitmentId, MoteCommitmentMemo>,
    ) -> Self {
        Self {
            peripheral,
            personal,
            commitments,
        }
    }

    pub fn as_ref(&'source self) -> MotesState<'source> {
        MotesState::new(
            self.peripheral,
            self.personal,
            self.commitments
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        )
    }
}
