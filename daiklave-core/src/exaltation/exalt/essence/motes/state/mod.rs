mod alias;
mod memo;

pub(crate) use alias::{PeripheralCommitted, PersonalCommitted};

use std::collections::HashMap;

use crate::exaltation::exalt::essence::{MoteCommitment, MoteCommitmentName};

pub(crate) use self::memo::MotesStateMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MotesState<'source> {
    pub peripheral_available: u8,
    pub peripheral_spent: u8,
    pub personal_available: u8,
    pub personal_spent: u8,
    pub other_commitments: HashMap<&'source str, (PeripheralCommitted, PersonalCommitted)>,
}

impl<'source> MotesState<'source> {
    pub fn commitments(&self) -> impl Iterator<Item = MoteCommitment<'source>> + '_ {
        self.other_commitments
            .iter()
            .map(|(&name, &(peripheral, personal))| MoteCommitment {
                name: MoteCommitmentName::Other(name),
                peripheral,
                personal,
            })
    }
}

impl<'source> From<&'source MotesStateMemo> for MotesState<'source> {
    fn from(memo: &'source MotesStateMemo) -> Self {
        Self {
            peripheral_available: memo.peripheral_available,
            peripheral_spent: memo.peripheral_spent,
            personal_available: memo.personal_available,
            personal_spent: memo.personal_spent,
            other_commitments: memo
                .other_commitments
                .iter()
                .map(|(name, (peripheral, personal))| (name.as_str(), (*peripheral, *personal)))
                .collect(),
        }
    }
}
