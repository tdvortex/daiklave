mod name;
pub use name::{MoteCommitmentName, MoteCommitmentNameMutation, OtherMoteCommitmentName};

/// A single committed mote effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoteCommitment<'source> {
    pub(crate) name: MoteCommitmentName<'source>,
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}

impl<'source> MoteCommitment<'source> {
    pub fn name(&self) -> MoteCommitmentName<'source> {
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
