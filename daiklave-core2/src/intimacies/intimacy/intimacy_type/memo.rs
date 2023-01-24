use serde::{Serialize, Deserialize};

use super::IntimacyType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum IntimacyTypeMemo {
    Tie(String),
    Principle,
}

impl<'source> IntimacyTypeMemo {
    pub fn as_ref(&'source self) -> IntimacyType<'source> {
        match self {
            IntimacyTypeMemo::Tie(target) => IntimacyType::Tie(target.as_str()),
            IntimacyTypeMemo::Principle => IntimacyType::Principle,
        }
    }
}