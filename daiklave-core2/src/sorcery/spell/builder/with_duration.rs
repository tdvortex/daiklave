use std::{collections::HashSet, num::NonZeroU8};

use crate::{book_reference::BookReference, sorcery::{spell::{cost::SpellCost, SpellKeyword}}};

use super::SpellBuilderWithDescription;

/// A Spell builder after the duration has been specified.
pub struct SpellBuilderWithDuration {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) control_spell_description: Option<String>,
    pub(crate) distortion: Option<(NonZeroU8, String)>,
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

    /// Provides a description for the spell's effect.
    pub fn description(self, description: String) -> SpellBuilderWithDescription {
        SpellBuilderWithDescription {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            cost: self.cost,
            duration: self.duration,
            description,
            keywords: self.keywords,
            control_spell_description: self.control_spell_description,
            distortion: self.distortion,
        }
    }
}