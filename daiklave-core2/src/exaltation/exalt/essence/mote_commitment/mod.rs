mod id;
mod name;

pub use id::{MoteCommitmentId};
pub use name::UncommitMotes;
use serde::{Serialize, Deserialize};


/// A single committed mote effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoteCommitment {
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}

impl MoteCommitment {
    /// The number of peripheral motes committed to the effect.
    pub fn peripheral(&self) -> u8 {
        self.peripheral
    }

    /// The number of personal motes committed to the effect.
    pub fn personal(&self) -> u8 {
        self.personal
    }
}
