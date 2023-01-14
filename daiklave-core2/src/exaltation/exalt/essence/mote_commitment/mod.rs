mod id;
mod memo;

pub use id::{MoteCommitmentId, OtherMoteCommitmentId};
pub(crate) use memo::MoteCommitmentMemo;

/// A single committed mote effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoteCommitment<'source> {
    pub(crate) name: &'source str,
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}

impl<'source> MoteCommitment<'source> {
    pub(crate) fn as_memo(&self) -> MoteCommitmentMemo {
        MoteCommitmentMemo {
            name: self.name.to_owned(),
            peripheral: self.peripheral,
            personal: self.personal,
        }
    }

    /// The name of the effect.
    pub fn name(&self) -> &'source str {
        self.name
    }

    /// The number of peripheral motes committed to the effect.
    pub fn peripheral(&self) -> u8 {
        self.peripheral
    }

    /// The number of personal motes committed to the effect.
    pub fn personal(&self) -> u8 {
        self.personal
    }
}
