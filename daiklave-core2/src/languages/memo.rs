use std::collections::HashSet;

use super::language::LanguageMutation;

pub(crate) struct LanguagesMemo {
    pub native_language: LanguageMutation,
    pub other_languages: HashSet<LanguageMutation>,
}