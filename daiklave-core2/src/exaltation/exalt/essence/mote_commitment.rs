use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) struct MoteCommitment {
    pub name: String,
    pub peripheral: u8,
    pub personal: u8,
}
