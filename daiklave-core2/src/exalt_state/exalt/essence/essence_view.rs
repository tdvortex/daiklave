use std::collections::HashMap;

use super::{CommittedMotesId, MoteState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MoteCommitmentView<'source> {
    pub(crate) name: &'source str,
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EssenceView<'source> {
    pub(crate) rating: u8,
    pub(crate) motes: MotesView<'source>,
}

impl<'source> EssenceView<'source> {
    pub fn rating(&self) -> u8 {
        self.rating
    }

    pub fn motes(&self) -> &MotesView {
        &self.motes
    }

    pub fn motes_mut(&mut self) -> &mut MotesView<'source> {
        &mut self.motes
    }
}

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
