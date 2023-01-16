use crate::{languages::Languages, Character};

impl<'view, 'source> Character<'source> {
    /// Get all languages spoken by the character.
    pub fn languages(&'view self) -> &'view Languages<'source> {
        &self.languages
    }
}
