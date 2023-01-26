use serde::{Deserialize, Serialize};

use super::IntimacyType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum IntimacyTypeMemo {
    Tie(String, String),
    Principle(String),
}

impl<'source> IntimacyTypeMemo {
    pub(crate) fn as_ref(&'source self) -> IntimacyType<'source> {
        match self {
            IntimacyTypeMemo::Tie(target, description) => {
                IntimacyType::Tie(target.as_str(), description.as_str())
            }
            IntimacyTypeMemo::Principle(description) => {
                IntimacyType::Principle(description.as_str())
            }
        }
    }
}
