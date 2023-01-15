pub mod language;
mod memo;
pub(crate) use memo::LanguagesMemo;

use std::collections::HashSet;

use crate::merits::merit::{Merit, MeritSource};

use self::language::Language;

/// The languages spoken by a character.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Languages<'source> {
    pub(crate) native_language: Language<'source>,
    pub(crate) other_languages: HashSet<Language<'source>>,
}

impl<'source> Languages<'source> {
    pub(crate) fn as_memo(&self) -> LanguagesMemo {
        LanguagesMemo { native_language: self.native_language.as_memo(), other_languages: self.other_languages.iter().map(|view| view.as_memo()).collect() }
    }


    /// The character's native language.
    pub fn native_language(&self) -> Language<'source> {
        self.native_language
    }

    /// Iterates over all spoken languages the character knows, as a pair of (language, is_native).
    pub fn iter(&self) -> impl Iterator<Item = (Language<'source>, bool)> + '_ {
        std::iter::once((self.native_language, true)).chain(self.other_languages.iter().copied().map(|language| (language, false)))
    }

    pub(crate) fn iter_merits(&self) -> impl Iterator<Item = Merit<'source>> {
        let mut major_language_merits = Vec::new();
        let mut local_tongues_count = 0;

        for language in self.other_languages.iter().copied() {
            match language {
                Language::MajorLanguage(major) => {major_language_merits.push(Merit(MeritSource::MajorLanguage(major)));}
                Language::LocalTongue(_) => {local_tongues_count += 1;}
            }
        }

        let local_tongues_merit_iter =if local_tongues_count > 0 {
            vec![Merit(MeritSource::LocalTongues(local_tongues_count))].into_iter()
        } else {
            vec![].into_iter()
        };

        major_language_merits.into_iter().chain(local_tongues_merit_iter)
    }
}