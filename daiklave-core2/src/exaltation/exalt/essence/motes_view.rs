use std::collections::HashMap;

use super::{mote_commitment_view::MoteCommitmentView, mote_pool::MotePool, MoteCommitmentId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotesView<'source> {
    pub(crate) peripheral: MotePool,
    pub(crate) personal: MotePool,
    pub(crate) commitments: HashMap<MoteCommitmentId, MoteCommitmentView<'source>>,
}

impl<'source> MotesView<'source> {
    pub fn peripheral(&self) -> &MotePool {
        &self.peripheral
    }

    pub fn peripheral_mut(&mut self) -> &mut MotePool {
        &mut self.peripheral
    }

    pub fn personal(&self) -> &MotePool {
        &self.personal
    }

    pub fn personal_mut(&mut self) -> &mut MotePool {
        &mut self.personal
    }

    pub fn committed(&self) -> impl Iterator<Item = (MoteCommitmentId, &str, u8, u8)> {
        self.commitments
            .iter()
            .map(|(k, v)| (*k, v.name, v.peripheral, v.personal))
    }
}
