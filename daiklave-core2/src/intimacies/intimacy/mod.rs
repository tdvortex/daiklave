mod error;
mod intimacy_type;
mod level;
mod mutation;

pub use error::IntimacyError;
pub use intimacy_type::IntimacyType;
pub(crate) use intimacy_type::IntimacyTypeMemo;
pub use level::IntimacyLevel;
pub use mutation::IntimacyMutation;

/// An Intimacy held by a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Intimacy<'source> {
    pub(crate) intimacy_type: IntimacyType<'source>,
    pub(crate) level: IntimacyLevel,
}

impl<'source> Intimacy<'source> {
    /// The type of the Intimacy. ("type" is a reserved keyword in Rust.)
    pub fn intimacy_type(&self) -> IntimacyType<'source> {
        self.intimacy_type
    }

    /// The level of the Intimacy.
    pub fn level(&self) -> IntimacyLevel {
        self.level
    }

    /// If the intimacy is a Tie, what the Tie is towards.
    pub fn tie_to(&self) -> Option<&'source str> {
        if let IntimacyType::Tie(tie_to, _) = self.intimacy_type {
            Some(tie_to)
        } else {
            None
        }
    }

    /// The description of the Intimacy. This either describes the emotional
    /// quality of a Tie, or is a statement of the Principle.
    pub fn description(&self) -> &'source str {
        match self.intimacy_type {
            IntimacyType::Tie(_, description) | IntimacyType::Principle(description) => description,
        }
    }
}
