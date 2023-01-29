use serde::{Deserialize, Serialize};

use super::{MajorLanguage, LocalTongueName, Language};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageMutation {
    MajorLanguage(MajorLanguage),
    LocalTongue(LocalTongueName),
}

impl Default for LanguageMutation {
    fn default() -> Self {
        LanguageMutation::MajorLanguage(MajorLanguage::default())
    }
}

impl From<Language<'_>> for LanguageMutation {
    fn from(language: Language<'_>) -> Self {
         match language {
            Language::MajorLanguage(major) => Self::MajorLanguage(major),
            Language::LocalTongue(local) => Self::LocalTongue(local.into()),
        }
    }
}

impl From<MajorLanguage> for LanguageMutation {
    fn from(major_language: MajorLanguage) -> Self {
        Self::MajorLanguage(major_language)
    }
}

impl From<LocalTongueName> for LanguageMutation {
    fn from(local_tongue: LocalTongueName) -> Self {
        Self::LocalTongue(local_tongue)
    }
}