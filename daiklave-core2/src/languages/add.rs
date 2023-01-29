use crate::{CharacterMutation};

use super::language::{MajorLanguage, LocalTongueName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddLanguages {
    pub major_languages: Vec<MajorLanguage>,
    pub local_tongues: Vec<LocalTongueName>,
}

impl From<AddLanguages> for CharacterMutation {
    fn from(add_languages: AddLanguages) -> Self {
        Self::AddMerit(add_languages.into())
    }
}