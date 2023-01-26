mod memo;
pub(crate) use memo::IntimacyTypeMemo;

/// The type of an intimacy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntimacyType<'source> {
    /// An emotional connection to a person, place or thing, which may be
    /// positive or negative.
    Tie(&'source str, &'source str),
    /// A belief the character holds.
    Principle(&'source str),
}

impl<'source> IntimacyType<'source> {
    pub(crate) fn as_memo(&self) -> IntimacyTypeMemo {
        match self {
            IntimacyType::Tie(target, description) => {
                IntimacyTypeMemo::Tie((*target).to_owned(), (*description).to_owned())
            }
            IntimacyType::Principle(description) => {
                IntimacyTypeMemo::Principle((*description).to_owned())
            }
        }
    }
}
