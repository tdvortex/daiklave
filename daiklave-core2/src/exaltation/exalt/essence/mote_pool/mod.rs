mod name;
pub use name::MotePoolName;

use super::{MoteCommitment};

/// The available and spent motes from either a peripheral or personal pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotePool<'source> {
    pub(crate) name: MotePoolName,
    pub(crate) available: u8,
    pub(crate) spent: u8,
    pub(crate) commitments: Vec<MoteCommitment<'source>>,
}

impl<'source> MotePool<'source> {
    pub fn name(&self) -> MotePoolName {
        self.name
    }

    /// The available motes from the specific pool.
    pub fn available(&self) -> u8 {
        self.available
    }

    /// The spent (but not committed) motes from the specific pool.
    pub fn spent(&self) -> u8 {
        self.spent
    }

    /// The sum of the committed motes from the specific pool.
    pub fn committed(&self) -> u8 {
        self.commitments.iter().fold(0, |sum, commitment| {
            match self.name {
                MotePoolName::Peripheral => sum+commitment.peripheral,
                MotePoolName::Personal => sum+commitment.personal
            }
        })
    }

    /// All active mote commitment effects on this pool.
    pub fn commitments(&self) -> impl Iterator<Item = MoteCommitment<'source>> + '_ {
        self.commitments.iter().copied()
    }
}
