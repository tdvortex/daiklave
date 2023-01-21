use std::{num::NonZeroU8, collections::HashSet};

use crate::{book_reference::BookReference, sorcery::{SorceryCircle, spell::{cost::{SpellCost, SpellMotesCost}, SpellKeyword}}};

use super::SpellBuilderWithWillpower;

pub struct SpellBuilderWithMoteCost {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpellKeyword>,
    pub(crate) circle: SorceryCircle,
    pub(crate) mote_cost: u8,
}

impl SpellBuilderWithMoteCost {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn willpower(self, willpower: NonZeroU8) -> SpellBuilderWithWillpower {
        let cost = SpellCost {
            motes_cost: NonZeroU8::new(self.mote_cost).map_or(SpellMotesCost::Ritual, SpellMotesCost::SorcerousMotes),
            willpower_cost: willpower,
        };

        SpellBuilderWithWillpower {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            circle: self.circle,
            cost,
            keywords: self.keywords
        }
    }
}