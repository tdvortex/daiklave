use crate::{
    languages::{language::LanguageMutation, LanguageError, Languages},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Get all languages spoken by the character.
    pub fn languages(&'view self) -> &'view Languages<'source> {
        &self.languages
    }

    /// Add a (non-native) language to the character.
    pub fn add_language(
        &mut self,
        language_mutation: &'source LanguageMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        let language = language_mutation.as_ref();

        if self.languages.native_language == language
            || self.languages.other_languages.insert(language)
        {
            Err(CharacterMutationError::LanguageError(
                LanguageError::DuplicateLanguage,
            ))
        } else {
            Ok(self)
        }
    }

    /// Checks if a language can be added as a non-native language.
    pub fn check_add_language(
        &self,
        language_mutation: &'source LanguageMutation,
    ) -> Result<(), CharacterMutationError> {
        let language = language_mutation.as_ref();

        if self.languages.native_language == language
            || self.languages.other_languages.contains(&language)
        {
            Err(CharacterMutationError::LanguageError(
                LanguageError::DuplicateLanguage,
            ))
        } else {
            Ok(())
        }
    }

    /// Checks if the character's native language can be set to the specified language.
    pub fn check_set_native_language(
        &self,
        language_mutation: &'source LanguageMutation,
    ) -> Result<(), CharacterMutationError> {
        let language = language_mutation.as_ref();

        if self.languages.native_language == language
            || self.languages.other_languages.contains(&language)
        {
            Err(CharacterMutationError::LanguageError(
                LanguageError::DuplicateLanguage,
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the character's native language. This will override the previous
    /// native language.
    pub fn set_native_language(
        &mut self,
        language_mutation: &'source LanguageMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        let language = language_mutation.as_ref();

        if self.languages.native_language == language
            || self.languages.other_languages.contains(&language)
        {
            Err(CharacterMutationError::LanguageError(
                LanguageError::DuplicateLanguage,
            ))
        } else {
            self.languages.native_language = language;
            Ok(self)
        }
    }

    /// Checks if a language can be removed.
    pub fn check_remove_language(
        &self,
        language_mutation: &'source LanguageMutation,
    ) -> Result<(), CharacterMutationError> {
        let language = language_mutation.as_ref();

        if self.languages.native_language == language {
            Err(CharacterMutationError::LanguageError(
                LanguageError::RemoveNative,
            ))
        } else if !self.languages.other_languages.contains(&language) {
            Err(CharacterMutationError::LanguageError(
                LanguageError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    /// Removes a language from the character.
    pub fn remove_language(
        &mut self,
        language_mutation: &'source LanguageMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        let language = language_mutation.as_ref();

        if self.languages.native_language == language {
            Err(CharacterMutationError::LanguageError(
                LanguageError::RemoveNative,
            ))
        } else if !self.languages.other_languages.remove(&language) {
            Err(CharacterMutationError::LanguageError(
                LanguageError::NotFound,
            ))
        } else {
            Ok(self)
        }
    }
}
