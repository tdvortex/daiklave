use std::{collections::HashSet, num::NonZeroU8};

use crate::{
    book_reference::BookReference,
    sorcery::spell::{cost::SpellCost, SpellKeyword, SpellName},
};

use super::SpellBuilderWithDuration;

/// A Spell builder after the willpower cost has been specified.
pub struct SpellBuilderWithWillpower {
    pub(crate) name: SpellName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) control_spell_description: Option<String>,
    pub(crate) distortion: Option<(NonZeroU8, String)>,
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

    /// Describes the control spell bonus of the Spell, if any.
    pub fn control_spell_description(mut self, description: String) -> Self {
        self.control_spell_description = Some(description);
        self
    }

    /// Describes the methods opposing sorcerers may use to distort this spell.
    pub fn distortion(mut self, goal_number: NonZeroU8, description: String) -> Self {
        self.distortion = Some((goal_number, description));
        self
    }

    /// Sets the duration of the Spell's effects. Often "Instant" but may
    /// be any defined string, such as "Until the next full moon".
    pub fn duration(self, duration: String) -> SpellBuilderWithDuration {
        SpellBuilderWithDuration {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            cost: self.cost,
            duration,
            keywords: self.keywords,
            control_spell_description: self.control_spell_description,
            distortion: self.distortion,
        }
    }
}
