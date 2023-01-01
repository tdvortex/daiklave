use super::MoteCommitmentMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MoteCommitmentView<'source> {
    pub(crate) name: &'source str,
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}

impl<'source> MoteCommitmentView<'source> {
    pub fn as_memo(&self) -> MoteCommitmentMemo {
        MoteCommitmentMemo {
            name: self.name.to_owned(),
            peripheral: self.peripheral,
            personal: self.personal,
        }
    }
}
