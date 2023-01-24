use super::{IntimacyType, IntimacyLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IntimacyInner<'source> {
    pub intimacy_type: IntimacyType<'source>,
    pub intimacy_level: IntimacyLevel,
    pub description: &'source str,
}