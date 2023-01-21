use std::{num::NonZeroU8, collections::HashSet};

use crate::{sorcery::{SorceryCircle, spell::SpellKeyword}, book_reference::BookReference};

use super::SpellBuilderWithMoteCost;

/// A Spell builder after its Circle has been specified.
pub struct SpellBuilderWithCircle {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
}

impl SpellBuilderWithCircle {
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

    /// Sets a Sorcerous Motes cost to cast this spell.
    pub fn sorcerous_motes(self, sorcerous_motes: NonZeroU8) -> SpellBuilderWithMoteCost {
        SpellBuilderWithMoteCost {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            circle: self.circle,
            mote_cost: sorcerous_motes.get(),
            keywords: self.keywords,
        }
    }

    /// Defines this spell as a Ritual; this costs no explicit Sorcerous Motes
    /// to use, but can only be cast outside of combat.
    pub fn ritual(self) -> SpellBuilderWithMoteCost {
        SpellBuilderWithMoteCost {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            circle: self.circle,
            mote_cost: 0,
            keywords: self.keywords,
        }
    }
}