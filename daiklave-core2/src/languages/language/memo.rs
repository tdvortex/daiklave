use super::MajorLanguage;

/// The details of a language to be added or removed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageMutation {
    MajorLanguage(MajorLanguage),
    LocalTongue(String),
}