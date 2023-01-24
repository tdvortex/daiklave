use super::{IntimacyId, IntimacyTypeMemo, IntimacyLevel};

/// An Intimacy to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntimacyMutation {
    id: IntimacyId,
    intimacy_type: IntimacyTypeMemo,
    intimacy_level: IntimacyLevel,
    description: String
}