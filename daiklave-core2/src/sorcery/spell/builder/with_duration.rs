use std::collections::HashSet;

use crate::{book_reference::BookReference, sorcery::{SorceryCircle, spell::{cost::SpellCost, SpellKeyword}}};

use super::SpellBuilderWithDescription;

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
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

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