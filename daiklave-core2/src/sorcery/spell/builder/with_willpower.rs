use std::collections::HashSet;

use crate::{sorcery::{spell::{cost::SpellCost, SpellKeyword}, SorceryCircle}, book_reference::BookReference};

use super::SpellBuilderWithDuration;

/// A Spell builder after the willpower cost has been specified.
pub struct SpellBuilderWithWillpower {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
    pub(crate) cost: SpellCost,
}

impl SpellBuilderWithWillpower {
    /// Sets the book reference for this Spell.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Provides a short summary of the Spell.
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Adds a keyword to the Spell.
    pub fn keyword(mut self, keyword: SpellKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    /// Sets the duration of the Spell's effects. Often "Instant" but may
    /// be any defined string, such as "Until the next full moon".
    pub fn duration(self, duration: String) -> SpellBuilderWithDuration {
        SpellBuilderWithDuration {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            circle: self.circle,
            cost: self.cost,
            duration,
            keywords: self.keywords,
        }
    }
}