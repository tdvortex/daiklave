use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::{Language, LanguageMutation, LocalTongueName, MajorLanguage};

/// A mutation to remove a language from the character. Native languages
/// cannot be removed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveLanguage(pub(crate) LanguageMutation);

impl RemoveLanguage {
    /// Remove a major language.
    pub fn major_language(major_language: MajorLanguage) -> Self {
        Self(LanguageMutation::MajorLanguage(major_language))
    }

    /// Remove a local tongue from the character.
    pub fn local_tongue(local_tongue: LocalTongueName) -> Self {
        Self(LanguageMutation::LocalTongue(local_tongue))
    }
}

impl From<LanguageMutation> for RemoveLanguage {
    fn from(mutation: LanguageMutation) -> Self {
        Self(mutation)
    }
}

impl From<Language<'_>> for RemoveLanguage {
    fn from(language: Language<'_>) -> Self {
        Self(LanguageMutation::from(language))
    }
}

impl From<MajorLanguage> for RemoveLanguage {
    fn from(major: MajorLanguage) -> Self {
        Self(LanguageMutation::from(major))
    }
}

impl From<LocalTongueName> for RemoveLanguage {
    fn from(local_tongue_name: LocalTongueName) -> Self {
        Self(LanguageMutation::from(local_tongue_name))
    }
}

impl From<RemoveLanguage> for CharacterMutation {
    fn from(remove_language: RemoveLanguage) -> Self {
        Self::RemoveLanguage(remove_language)
    }
}
