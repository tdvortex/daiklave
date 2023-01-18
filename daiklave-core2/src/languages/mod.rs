mod error;
pub use error::LanguageError;
/// Details of individual languages.
pub mod language;
mod memo;
pub(crate) use memo::LanguagesMemo;

use std::collections::HashSet;

use self::language::Language;

/// The languages spoken by a character.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Languages<'source> {
    pub(crate) native_language: Language<'source>,
    pub(crate) other_languages: HashSet<Language<'source>>,
}

impl<'source> Languages<'source> {
    pub(crate) fn as_memo(&self) -> LanguagesMemo {
        LanguagesMemo {
            native_language: self.native_language.as_memo(),
            other_languages: self
                .other_languages
                .iter()
                .map(|view| view.as_memo())
                .collect(),
        }
    }

    /// The character's native language.
    pub fn native_language(&self) -> Language<'source> {
        self.native_language
    }

    /// Iterates over all spoken languages the character knows, as a pair of (language, is_native).
    pub fn iter(&self) -> impl Iterator<Item = (Language<'source>, bool)> + '_ {
        std::iter::once((self.native_language, true)).chain(
            self.other_languages
                .iter()
                .copied()
                .map(|language| (language, false)),
        )
    }
}
