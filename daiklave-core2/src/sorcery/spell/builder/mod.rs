mod with_circle;
mod with_description;
mod with_duration;
mod with_mote_cost;
mod with_willpower;
use std::collections::HashSet;

pub use with_circle::SpellBuilderWithCircle;
pub use with_description::SpellBuilderWithDescription;
pub use with_duration::SpellBuilderWithDuration;
pub use with_mote_cost::SpellBuilderWithMoteCost;
pub use with_willpower::SpellBuilderWithWillpower;

use crate::{book_reference::BookReference, sorcery::SorceryCircle};

use super::SpellKeyword;

/// Builder for a Spell. Required fields: name (already specified),
/// circle, sorcerous motes (or ritual), willpower cost (1+), duration, and
/// description. Optional fields: book reference, summary.
pub struct SpellBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
}

impl SpellBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn circle(self, circle: SorceryCircle) -> SpellBuilderWithCircle {
        SpellBuilderWithCircle {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            circle,
            keywords: self.keywords,
        }
    }
}