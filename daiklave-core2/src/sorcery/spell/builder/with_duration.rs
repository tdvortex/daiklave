use std::collections::HashSet;

use crate::{book_reference::BookReference, sorcery::{SorceryCircle, spell::{cost::SpellCost, SpellKeyword}}};

use super::SpellBuilderWithDescription;

/// A Spell builder after the duration has been specified.
pub struct SpellBuilderWithDuration {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
    pub(crate) cost: SpellCost,
    pub(crate) duration: String,
}

impl SpellBuilderWithDuration {
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

    /// Provides a description for the spell's effect.
    pub fn description(self, description: String) -> SpellBuilderWithDescription {
        SpellBuilderWithDescription {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            circle: self.circle,
            cost: self.cost,
            duration: self.duration,
            description,
            keywords: self.keywords
        }
    }
}