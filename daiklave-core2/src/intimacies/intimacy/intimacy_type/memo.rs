use serde::{Deserialize, Serialize};

use super::IntimacyType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum IntimacyTypeMemo {
    Tie(String, String),
    Principle(String),
}

impl From<IntimacyType<'_>> for IntimacyTypeMemo {
    fn from(view: IntimacyType<'_>) -> Self {
        match view {
            IntimacyType::Tie(target, emotion) => Self::Tie(target.to_owned(), emotion.to_owned()),
            IntimacyType::Principle(description) => Self::Principle(description.to_owned()),
        }
    }
}

impl<'source> Into<IntimacyType<'source>> for &'source IntimacyTypeMemo {
    fn into(self) -> IntimacyType<'source> {
        match self {
            IntimacyTypeMemo::Tie(target, emotion) => IntimacyType::Tie(&target, &emotion),
            IntimacyTypeMemo::Principle(description) => IntimacyType::Principle(&description),
        }
    }
}