use thiserror::Error;

/// An error related to character languages
#[derive(Debug, Error)]
pub enum LanguageError {
    /// Languages can only be learned once
    #[error("Languages can only be learned once")]
    DuplicateLanguage,
    /// Could not find specified language
    #[error("Language not found")]
    NotFound,
    /// Native languages cannot be removed
    #[error("Native languages cannot be removed")]
    RemoveNative,
}