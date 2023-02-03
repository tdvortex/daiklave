use serde::{Deserialize, Serialize};

use crate::CharacterMutation;

use super::{LanguageMutation, LocalTongueName, MajorLanguage};

/// A mutation to set the native language of the character.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SetNativeLanguage(pub(crate) LanguageMutation);

impl SetNativeLanguage {
    /// Sets the character's native language to one of the major languages of
    /// Creation.
    pub fn major_language(major_language: MajorLanguage) -> Self {
        Self(LanguageMutation::MajorLanguage(major_language))
    }

    /// Sets the character's native language to a local language from a
    /// smaller community.
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
