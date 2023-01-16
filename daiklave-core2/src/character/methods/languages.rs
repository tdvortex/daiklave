use crate::{Character, languages::Languages};

impl<'view, 'source> Character<'source> {
    /// Get all languages spoken by the character.
    pub fn languages(&'view self) -> &'view Languages<'source> {
        &self.languages
    }
}