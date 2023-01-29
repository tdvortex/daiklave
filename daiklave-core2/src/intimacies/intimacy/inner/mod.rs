mod memo;
pub(crate) use memo::IntimacyInnerMemo;

use super::{IntimacyLevel, IntimacyType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IntimacyInner<'source> {
    pub intimacy_type: IntimacyType<'source>,
    pub intimacy_level: IntimacyLevel,
    pub description: &'source str,
}