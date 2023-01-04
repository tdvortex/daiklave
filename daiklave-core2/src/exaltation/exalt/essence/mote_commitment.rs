use super::MoteCommitmentMemo;

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
}
