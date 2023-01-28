use super::{IntimacyLevel, IntimacyTypeMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntimacyMemo {
    pub(crate) intimacy_type: IntimacyTypeMemo,
    pub(crate) level: IntimacyLevel,
}
