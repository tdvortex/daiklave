mod with_circle;
mod with_description;
mod with_duration;
mod with_mote_cost;
mod with_willpower;
use std::{collections::HashSet, num::NonZeroU8};

pub use with_circle::SpellBuilderWithCircle;
pub use with_description::SpellBuilderWithDescription;
pub use with_duration::SpellBuilderWithDuration;
pub use with_mote_cost::SpellBuilderWithMoteCost;
pub use with_willpower::SpellBuilderWithWillpower;

use crate::{book_reference::BookReference, sorcery::SorceryCircle};

use super::SpellKeyword;

/// Builder for a Spell. Required fields: name (already specified),
/// circle, sorcerous motes (or ritual), willpower cost (1+), duration, and
/// description. Optional fields: book reference, summary, keywords, control
/// spell description, and distortion description.
pub struct SpellBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) control_spell_description: Option<String>,
    pub(crate) distortion: Option<(NonZeroU8, String)>,
}

impl SpellBuilder {
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

    /// Sets the book reference for this Spell.
    pub fn circle(self, circle: SorceryCircle) -> SpellBuilderWithCircle {
        SpellBuilderWithCircle {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            keywords: self.keywords,
            control_spell_description: self.control_spell_description,
            distortion: self.distortion,
            circle,
        }
    }
}