use std::collections::HashSet;

use crate::{sorcery::{spell::{cost::SpellCost, SpellKeyword}, SorceryCircle}, book_reference::BookReference};

use super::SpellBuilderWithDuration;

pub struct SpellBuilderWithWillpower {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
    pub(crate) cost: SpellCost,
}

impl SpellBuilderWithWillpower {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

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