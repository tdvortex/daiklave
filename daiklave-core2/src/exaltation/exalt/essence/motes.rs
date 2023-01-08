use std::collections::HashMap;

use super::{
    mote_commitment::MoteCommitment, mote_pool::MotePool, motes_memo::MotesMemo, MoteCommitmentId,
};

/// The current state of an exalt's mote pools and commitments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Motes<'source> {
    peripheral: MotePool,
    personal: MotePool,
    commitments: HashMap<MoteCommitmentId, MoteCommitment<'source>>,
}

impl<'source> Motes<'source> {
    pub(in crate::exaltation::exalt::essence) fn new(
        peripheral: MotePool,
        personal: MotePool,
        commitments: HashMap<MoteCommitmentId, MoteCommitment<'source>>,
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

    /// The exalt's current Peripheral mote pool.
    pub fn peripheral(&self) -> &MotePool {
        &self.peripheral
    }

    pub(crate) fn peripheral_mut(&mut self) -> &mut MotePool {
        &mut self.peripheral
    }

    /// The exalt's current Personal mote pool.
    pub fn personal(&self) -> &MotePool {
        &self.personal
    }

    pub(crate) fn personal_mut(&mut self) -> &mut MotePool {
        &mut self.personal
    }

    /// The exalt's current mote commitments.
    pub fn committed(&self) -> impl Iterator<Item = (MoteCommitmentId, MoteCommitment)> {
        self.commitments.iter().map(|(k, v)| (*k, *v))
    }

    pub(crate) fn commitments(&self) -> &HashMap<MoteCommitmentId, MoteCommitment<'source>> {
        &self.commitments
    }

    pub(crate) fn commitments_mut(
        &mut self,
    ) -> &mut HashMap<MoteCommitmentId, MoteCommitment<'source>> {
        &mut self.commitments
    }
}
