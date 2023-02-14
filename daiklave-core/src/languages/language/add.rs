use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::{Language, LanguageMutation, LocalTongueName, MajorLanguage};

/// A mutation to add a language to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddLanguage(pub(crate) LanguageMutation);

impl AddLanguage {
    /// Add a major language of Creation.
    pub fn major_language(major_language: MajorLanguage) -> Self {
        Self(LanguageMutation::MajorLanguage(major_language))
    }

    /// Add a local tongue to the character.
    pub fn local_tongue(local_tongue: impl Into<LocalTongueName>) -> Self {
        Self(LanguageMutation::LocalTongue(local_tongue.into()))
    }
}

impl From<LanguageMutation> for AddLanguage {
    fn from(mutation: LanguageMutation) -> Self {
        Self(mutation)
    }
}

impl From<Language<'_>> for AddLanguage {
    fn from(language: Language<'_>) -> Self {
        Self(LanguageMutation::from(language))
    }
}

impl From<MajorLanguage> for AddLanguage {
    fn from(major: MajorLanguage) -> Self {
        Self(LanguageMutation::from(major))
    }
}

impl From<LocalTongueName> for AddLanguage {
    fn from(local_tongue_name: LocalTongueName) -> Self {
        Self(LanguageMutation::from(local_tongue_name))
    }
}

impl From<AddLanguage> for CharacterMutation {
    fn from(add_language: AddLanguage) -> Self {
        Self::AddLanguage(add_language)
    }
}
