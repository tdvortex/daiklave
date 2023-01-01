use std::collections::HashMap;

use super::{MoteState, CommittedMotesId, mote_commitment_view::MoteCommitmentView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotesView<'source> {
    pub(crate) peripheral: MoteState,
    pub(crate) personal: MoteState,
    pub(crate) commitments: HashMap<CommittedMotesId, MoteCommitmentView<'source>>,
}

impl<'source> MotesView<'source> {
    pub fn peripheral(&self) -> &MoteState {
        &self.peripheral
    }

    pub fn peripheral_mut(&mut self) -> &mut MoteState {
        &mut self.peripheral
    }

    pub fn personal(&self) -> &MoteState {
        &self.personal
    }

    pub fn personal_mut(&mut self) -> &mut MoteState {
        &mut self.personal
    }

    pub fn committed(&self) -> impl Iterator<Item = (CommittedMotesId, &str, u8, u8)> {
        self.commitments
            .iter()
            .map(|(k, v)| (*k, v.name, v.peripheral, v.personal))
    }
}