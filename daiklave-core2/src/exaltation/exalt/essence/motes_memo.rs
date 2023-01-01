use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{mote_pool::MotePool, MoteCommitmentMemo, MoteCommitmentId};

/// The current state of a character's mote balances. Includes both peripheral
/// and personal mote pools, as well as commitments from both.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotesMemo {
    pub(crate) peripheral: MotePool,
    pub(crate) personal: MotePool,
    pub(crate) commitments: HashMap<MoteCommitmentId, MoteCommitmentMemo>,
}

impl MotesMemo {
    /// The character's current available and spent peripheral motes.
    pub fn peripheral(&self) -> &MotePool {
        &self.peripheral
    }

    pub(crate) fn peripheral_mut(&mut self) -> &mut MotePool {
        &mut self.peripheral
    }

    /// The character's current available and spent personal motes.
    pub fn personal(&self) -> &MotePool {
        &self.personal
    }

    pub(crate) fn personal_mut(&mut self) -> &mut MotePool {
        &mut self.personal
    }

    /// An iterator over the character's current mote commitments, structured
    /// as (id, name, peripheral committed, personal committed).
    pub fn committed(&self) -> impl Iterator<Item = (MoteCommitmentId, &str, u8, u8)> {
        self.commitments
            .iter()
            .map(|(k, v)| (*k, v.name.as_str(), v.peripheral, v.personal))
    }
}
