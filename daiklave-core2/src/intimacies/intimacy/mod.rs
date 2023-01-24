mod id;
mod inner;
mod intimacy_type;
mod level;
mod mutation;

pub use id::IntimacyId;
pub(crate) use inner::{IntimacyInner, IntimacyInnerMemo};
pub use intimacy_type::IntimacyType;
pub(crate) use intimacy_type::IntimacyTypeMemo;
pub use level::IntimacyLevel;
pub use mutation::IntimacyMutation;

/// An Intimacy held by a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Intimacy<'source> {
    pub(crate) id: IntimacyId,
    pub(crate) inner: IntimacyInner<'source>,
}

impl<'source> Intimacy<'source> {
    /// The Id of the Intimacy.
    pub fn id(&self) -> IntimacyId {
        self.id
    }

    /// The type of the Intimacy. ("type" is a reserved keyword in Rust.)
    pub fn intimacy_type(&self) -> IntimacyType<'source> {
        self.inner.intimacy_type
    }

    /// The level of the Intimacy.
    pub fn level(&self) -> IntimacyLevel {
        self.inner.intimacy_level
    }

    /// The description of the Intimacy. This either describes the emotional
    /// quality of a Tie, or is a statement of the Principle.
    pub fn description(&self) -> &'source str {
        self.inner.description
    }
}