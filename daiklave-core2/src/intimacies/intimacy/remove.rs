use crate::CharacterMutation;

use super::{IntimacyLevel, IntimacyTypeMemo};

/// An Intimacy to be removed from a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveIntimacy {
    pub(crate) intimacy_type: IntimacyTypeMemo,
    pub(crate) level: IntimacyLevel,
}

impl RemoveIntimacy {
    pub fn tie(target: impl ToString, emotion: impl ToString, level: IntimacyLevel) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Tie(target.to_string(), emotion.to_string()),
            level,
        }
    }

    pub fn principle(description: impl ToString, level: IntimacyLevel) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Principle(description.to_string()),
            level,
        }
    }
}

impl From<RemoveIntimacy> for CharacterMutation {
    fn from(remove_intimacy: RemoveIntimacy) -> Self {
        Self::RemoveIntimacy(remove_intimacy)
    }
}
