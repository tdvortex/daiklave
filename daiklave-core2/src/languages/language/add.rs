use crate::CharacterMutation;

use super::{LanguageMutation, MajorLanguage, LocalTongueName, Language};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddLanguage(pub(crate) LanguageMutation);

impl AddLanguage {
    pub fn major_language(major_language: MajorLanguage) -> Self {
        Self(LanguageMutation::MajorLanguage(major_language))
    }

    pub fn local_tongue(local_tongue: LocalTongueName) -> Self {
        Self(LanguageMutation::LocalTongue(local_tongue))
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