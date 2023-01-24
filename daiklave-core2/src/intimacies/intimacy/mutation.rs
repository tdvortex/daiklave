use super::{IntimacyId, inner::IntimacyInnerMemo};

/// An Intimacy to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntimacyMutation {
    pub(crate) id: IntimacyId,
    pub(crate) inner: IntimacyInnerMemo,
}