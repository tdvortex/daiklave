use crate::{artifact::AddArtifact, CharacterMutation};

use super::{Wonder, WonderName};

/// The name and details for a Wonder to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddWonder {
    pub(crate) name: WonderName,
    pub(crate) wonder: Wonder,
}

impl From<AddWonder> for CharacterMutation {
    fn from(add_wonder: AddWonder) -> Self {
        AddArtifact::from(add_wonder).into()
    }
}
