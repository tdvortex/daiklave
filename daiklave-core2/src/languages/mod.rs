mod add;
mod error;
pub use add::AddLanguages;
pub use error::LanguageError;
/// Details of individual languages.
pub mod language;
mod memo;
pub(crate) use memo::LanguagesMemo;

use crate::Character;

use self::language::{Language};

/// The languages spoken by a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Languages<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Languages<'view, 'source> {
    /// The character's native language.
    pub fn native_language(&self) -> Language<'source> {
        self.0.native_language.into()
    }

    /// Iterates over all spoken languages the character knows, as a pair of (language, is_native).
    pub fn iter(&self) -> impl Iterator<Item = (Language<'source>, bool)> + '_ {
        std::iter::once((self.0.native_language.into(), true)).chain(
            self.0.other_languages
                .iter()
                .map(|language| (language.into(), false)),
        )
    }
}
