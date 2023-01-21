use std::{collections::HashSet, num::NonZeroU8};

use crate::{book_reference::BookReference, sorcery::{spell::{cost::SpellCost, SpellInner, SpellKeyword, SpellMutation}, SorceryCircle}};

/// A Spell builder after the spell's description has been provided. To finish
/// the build, call build().
pub struct SpellBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) control_spell_description: Option<String>,
    pub(crate) distortion: Option<(NonZeroU8, String)>,
    pub(crate) circle: SorceryCircle,
    pub(crate) cost: SpellCost,
    pub(crate) duration: String,
    pub(crate) description: String,
}

impl SpellBuilderWithDescription {
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

    /// Completes the builder, returning the Spell as a CharmMutation.
    pub fn build(self) -> SpellMutation {
        let inner = SpellInner {
            name: self.name,
            summary: self.summary,
            cost: self.cost,
            duration: self.duration,
            description: self.description,
            book_reference: self.book_reference,
            keywords: self.keywords,
            control_spell_description: self.control_spell_description,
            distortion: self.distortion,
        };

        match self.circle {
            SorceryCircle::Terrestrial => SpellMutation::Terrestrial(inner.into()),
            SorceryCircle::Celestial => SpellMutation::Celestial(inner.into()),
            SorceryCircle::Solar => SpellMutation::Solar(inner.into()),
        }
    }
}