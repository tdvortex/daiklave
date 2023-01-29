mod memo;

use std::collections::HashMap;

use crate::exaltation::exalt::essence::{mote_commitment::MoteCommitment, mote_pool::MotePool};

pub(crate) use self::memo::MotesStateMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotesState<'source> {
    pub peripheral: MotePool,
    pub personal: MotePool,
    pub commitments: HashMap<&'source str, MoteCommitment>,
}

impl<'source> MotesState<'source> {
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
}
