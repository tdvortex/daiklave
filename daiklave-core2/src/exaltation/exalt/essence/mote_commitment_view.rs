#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MoteCommitmentView<'source> {
    pub(crate) name: &'source str,
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}
