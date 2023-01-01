use serde::{Deserialize, Serialize};

use super::MoteCommitmentView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) struct MoteCommitmentMemo {
    pub name: String,
    pub peripheral: u8,
    pub personal: u8,
}

impl<'source> MoteCommitmentMemo {
    pub fn as_ref(&'source self) -> MoteCommitmentView<'source> {
        MoteCommitmentView {
            name: self.name.as_str(),
            peripheral: self.peripheral,
            personal: self.personal,
        }
    }
}