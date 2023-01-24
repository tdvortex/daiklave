mod memo;
pub(crate) use memo::IntimacyInnerMemo;

use super::{IntimacyLevel, IntimacyType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IntimacyInner<'source> {
    pub intimacy_type: IntimacyType<'source>,
    pub intimacy_level: IntimacyLevel,
    pub description: &'source str,
}

impl<'source> IntimacyInner<'source> {
    pub fn as_memo(&self) -> IntimacyInnerMemo {
        IntimacyInnerMemo {
            intimacy_type: self.intimacy_type.as_memo(),
            intimacy_level: self.intimacy_level,
            description: self.description.to_owned(),
        }
    }
}
