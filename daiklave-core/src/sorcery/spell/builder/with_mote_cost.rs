use std::{collections::HashSet, num::NonZeroU8};

use crate::{
    book_reference::BookReference,
    sorcery::spell::{
        cost::{SpellCost, SpellMotesCost},
        SpellKeyword, SpellName,
    },
};

use super::SpellBuilderWithWillpower;

/// A Spell builder after the Sorcerous Mote cost has been specified (or set
/// to be a ritual).
pub struct SpellBuilderWithMoteCost {
    pub(crate) name: SpellName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) control_spell_description: Option<String>,
    pub(crate) distortion: Option<(NonZeroU8, String)>,
    pub(crate) mote_cost: u8,
}

impl SpellBuilderWithMoteCost {
    /// Sets the book reference for this Spell.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Provides a short summary of the Spell.
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Adds a keyword to the Spell.
    pub fn keyword(mut self, keyword: SpellKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    /// Describes the control spell bonus of the Spell, if any.
    pub fn control_spell_description(mut self, description: impl Into<String>) -> Self {
        self.control_spell_description = Some(description.into());
        self
    }

    /// Describes the methods opposing sorcerers may use to distort this spell.
    pub fn distortion(mut self, goal_number: NonZeroU8, description: impl Into<String>) -> Self {
        self.distortion = Some((goal_number, description.into()));
        self
    }

    /// Sets the Willpower cost to cast the spell.
    pub fn willpower(self, willpower: NonZeroU8) -> SpellBuilderWithWillpower {
        let cost = SpellCost {
            motes_cost: NonZeroU8::new(self.mote_cost)
                .map_or(SpellMotesCost::Ritual, SpellMotesCost::SorcerousMotes),
            willpower_cost: willpower,
        };

        SpellBuilderWithWillpower {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            cost,
            keywords: self.keywords,
            control_spell_description: self.control_spell_description,
            distortion: self.distortion,
        }
    }
}
