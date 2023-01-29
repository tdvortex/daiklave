use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::{MajorLanguage, LocalTongueName, LanguageMutation};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SetNativeLanguage(pub(crate) LanguageMutation);

impl SetNativeLanguage {
    pub fn major_language(major_language: MajorLanguage) -> Self {
        Self(LanguageMutation::MajorLanguage(major_language))
    }

    pub fn local_tongue(local_tongue: LocalTongueName) -> Self {
        Self(LanguageMutation::LocalTongue(local_tongue))
    }
}


impl From<MajorLanguage> for SetNativeLanguage {
    fn from(major_language: MajorLanguage) -> Self {
        Self::major_language(major_language)
    }
}

impl From<LocalTongueName> for SetNativeLanguage {
    fn from(local_tongue: LocalTongueName) -> Self {
        Self::local_tongue(local_tongue)
    }
}

impl From<SetNativeLanguage> for CharacterMutation {
    fn from(set_native_language: SetNativeLanguage) -> Self {
        Self::SetNativeLanguage(set_native_language)
    }
}