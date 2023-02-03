use crate::{
    languages::{
        language::{AddLanguage, RemoveLanguage, SetNativeLanguage},
        LanguageError, Languages,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Get all languages spoken by the character.
    pub fn languages(&'view self) -> Languages<'view, 'source> {
        Languages(self)
    }

    /// Adds a new (non-native) language to the character.
    pub fn add_language(
        &mut self,
        language: &'source AddLanguage,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.native_language == &language.0 || !self.other_languages.insert(&language.0) {
            Err(CharacterMutationError::LanguageError(
                LanguageError::DuplicateLanguage,
            ))
        } else {
            Ok(self)
        }
    }

    /// Sets the character's native language. This will override the previous
    /// native language.
    pub fn set_native_language(
        &mut self,
        set_native_language: &'source SetNativeLanguage,
    ) -> Result<&mut Self, CharacterMutationError> {
        let language = &set_native_language.0;

        if self.native_language == language || self.other_languages.contains(language) {
            Err(CharacterMutationError::LanguageError(
                LanguageError::DuplicateLanguage,
            ))
        } else {
            self.native_language = language;
            Ok(self)
        }
    }

    /// Removes a language from the character. Native languages cannot be
    /// removed.
    pub fn remove_language(
        &mut self,
        remove_language: &RemoveLanguage,
    ) -> Result<&mut Self, CharacterMutationError> {
        let language = &remove_language.0;

        if self.native_language == language {
            Err(CharacterMutationError::LanguageError(
                LanguageError::RemoveNative,
            ))
        } else if !self.other_languages.remove(language) {
            Err(CharacterMutationError::LanguageError(
                LanguageError::NotFound,
            ))
        } else {
            Ok(self)
        }
    }
}
