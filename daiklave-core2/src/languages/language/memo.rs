use serde::{Deserialize, Serialize};

use super::{Language, MajorLanguage};

/// The details of a language to be added or removed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageMutation {
    MajorLanguage(MajorLanguage),
    LocalTongue(String),
}

impl<'source> LanguageMutation {
    pub(crate) fn as_ref(&'source self) -> Language<'source> {
        match self {
            LanguageMutation::MajorLanguage(major) => Language::MajorLanguage(*major),
            LanguageMutation::LocalTongue(local) => Language::LocalTongue(local.as_str()),
        }
    }
}
