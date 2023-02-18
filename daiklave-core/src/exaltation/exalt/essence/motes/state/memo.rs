use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::mote_commitment::OtherMoteCommitmentName;

use super::{MotesState, PeripheralCommitted, PersonalCommitted};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MotesStateMemo {
    pub(crate) peripheral_available: u8,
    pub(crate) peripheral_spent: u8,
    pub(crate) personal_available: u8,
    pub(crate) personal_spent: u8,
    pub(crate) other_commitments:
        HashMap<OtherMoteCommitmentName, (PeripheralCommitted, PersonalCommitted)>,
}

impl From<&MotesState<'_>> for MotesStateMemo {
    fn from(motes: &MotesState<'_>) -> Self {
        Self {
            peripheral_available: motes.peripheral_available,
            peripheral_spent: motes.peripheral_spent,
            personal_available: motes.personal_available,
            personal_spent: motes.personal_spent,
            other_commitments: motes
                .other_commitments
                .iter()
                .map(|(name, (peripheral, personal))| ((*name).into(), (*peripheral, *personal)))
                .collect(),
        }
    }
}
