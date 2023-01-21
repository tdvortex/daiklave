use std::{num::NonZeroU8, collections::HashSet};

use crate::{sorcery::{SorceryCircle, spell::SpellKeyword}, book_reference::BookReference};

use super::SpellBuilderWithMoteCost;

pub struct SpellBuilderWithCircle {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
}

impl SpellBuilderWithCircle {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

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