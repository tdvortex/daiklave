use super::{IntimacyTypeMemo, IntimacyLevel};

/// An Intimacy to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntimacyMutation {
    pub(crate) intimacy_type: IntimacyTypeMemo,
    pub(crate) level: IntimacyLevel,
}
