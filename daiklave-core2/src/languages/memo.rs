use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{language::LanguageMutation, Languages};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LanguagesMemo {
    pub native_language: LanguageMutation,
    pub other_languages: HashSet<LanguageMutation>,
}

impl<'source> LanguagesMemo {
    pub fn as_ref(&'source self) -> Languages<'source> {
        Languages {
            native_language: self.native_language.as_ref(),
            other_languages: self
                .other_languages
                .iter()
                .map(|memo| memo.as_ref())
                .collect(),
        }
    }
}
