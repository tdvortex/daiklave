mod memo;

use std::collections::HashMap;

use crate::exaltation::exalt::essence::{
    mote_commitment::{MoteCommitment, MoteCommitmentId},
    mote_pool::MotePool,
};

pub(crate) use self::memo::MotesStateMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotesState<'source> {
    pub peripheral: MotePool,
    pub personal: MotePool,
    pub commitments: HashMap<MoteCommitmentId, MoteCommitment<'source>>,
}

impl<'source> MotesState<'source> {
    pub(crate) fn new(
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

    pub(crate) fn as_memo(&self) -> MotesStateMemo {
        MotesStateMemo::new(
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

    pub(crate) fn commitments(&self) -> &HashMap<MoteCommitmentId, MoteCommitment<'source>> {
        &self.commitments
    }

    pub(crate) fn commitments_mut(
        &mut self,
    ) -> &mut HashMap<MoteCommitmentId, MoteCommitment<'source>> {
        &mut self.commitments
    }
}
