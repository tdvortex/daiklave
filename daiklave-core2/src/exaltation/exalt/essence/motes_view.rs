use std::collections::HashMap;

use super::{
    mote_commitment_view::MoteCommitmentView, mote_pool::MotePool, motes_memo::MotesMemo,
    MoteCommitmentId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotesView<'source> {
    peripheral: MotePool,
    personal: MotePool,
    commitments: HashMap<MoteCommitmentId, MoteCommitmentView<'source>>,
}

impl<'source> MotesView<'source> {
    pub(in crate::exaltation::exalt::essence) fn new(
        peripheral: MotePool,
        personal: MotePool,
        commitments: HashMap<MoteCommitmentId, MoteCommitmentView<'source>>,
    ) -> Self {
        Self {
            peripheral,
            personal,
            commitments,
        }
    }

    pub(crate) fn as_memo(&self) -> MotesMemo {
        MotesMemo::new(
            self.peripheral,
            self.personal,
            self.commitments
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
        )
    }

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

    pub(crate) fn commitments(&self) -> &HashMap<MoteCommitmentId, MoteCommitmentView<'source>> {
        &self.commitments
    }

    pub(crate) fn commitments_mut(
        &mut self,
    ) -> &mut HashMap<MoteCommitmentId, MoteCommitmentView<'source>> {
        &mut self.commitments
    }
}
