use crate::CharacterMutation;

use super::{IntimacyLevel, IntimacyTypeMemo};

/// An Intimacy to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddIntimacy {
    pub(crate) intimacy_type: IntimacyTypeMemo,
    pub(crate) level: IntimacyLevel,
}

impl AddIntimacy {
    pub fn tie(target: impl Into<String>, emotion: impl Into<String>, level: IntimacyLevel) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Tie(target.into(), emotion.into()),
            level,
        }
    }

    pub fn principle(description: impl Into<String>, level: IntimacyLevel) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Principle(description.into()),
            level,
        }
    }
}

impl From<AddIntimacy> for CharacterMutation {
    fn from(add_intimacy: AddIntimacy) -> Self {
        Self::AddIntimacy(add_intimacy)
    }
}
