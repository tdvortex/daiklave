use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{language::LanguageMutation};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LanguagesMemo {
    pub native_language: LanguageMutation,
    pub other_languages: HashSet<LanguageMutation>,
}
