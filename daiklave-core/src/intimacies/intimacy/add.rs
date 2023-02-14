use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::{
    builder::{IntimacyBuilder, IntimacyBuilderWithDescription, TieBuilder},
    IntimacyLevel, IntimacyTypeMemo,
};

/// An Intimacy to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddIntimacy {
    pub(crate) intimacy_type: IntimacyTypeMemo,
    pub(crate) level: IntimacyLevel,
}

impl AddIntimacy {
    /// Starts building a Tie to add to the character.
    pub fn tie(target: impl Into<String>) -> TieBuilder {
        IntimacyBuilder::tie(target)
    }

    /// Starts building a Principle to add to the character.
    pub fn principle(description: impl Into<String>) -> IntimacyBuilderWithDescription {
        IntimacyBuilder::principle(description)
    }
}

impl From<AddIntimacy> for CharacterMutation {
    fn from(add_intimacy: AddIntimacy) -> Self {
        Self::AddIntimacy(add_intimacy)
    }
}
