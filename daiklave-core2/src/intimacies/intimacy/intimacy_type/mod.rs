mod memo;
pub(crate) use memo::IntimacyTypeMemo;

/// The type of an intimacy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntimacyType<'source> {
    /// An emotional connection to a person, place or thing, which may be
    /// positive or negative. The inner string described the thing the emotion
    /// pertains to.
    Tie(&'source str),
    /// A belief the character holds.
    Principle,
}

impl<'source> IntimacyType<'source> {
    pub(crate) fn as_memo(&self) -> IntimacyTypeMemo {
        match self {
            IntimacyType::Tie(target) => IntimacyTypeMemo::Tie((*target).to_owned()),
            IntimacyType::Principle => IntimacyTypeMemo::Principle,
        }
    }
}
