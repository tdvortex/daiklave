use std::collections::{HashSet, HashMap};

use crate::{book_reference::BookReference, sorcery::spell::builder::SpellBuilder};

use super::evocation::{builder::EvocationBuilder, EvokableId};

/// Builder for constructing a new Charm (or spell).
pub struct CharmBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
}

impl CharmBuilder {
    /// Defines the book reference for the Charm/Spell
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Gives the Charm/Spell a summary
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Constructs the Charm as a Spell
    pub fn spell(self) -> SpellBuilder {
        SpellBuilder {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            keywords: HashSet::new(),
            control_spell_description: None,    
            distortion: None,
        }
    }

    /// Constructs the Charm as an Evocation. Requires specifying what it is an
    /// evocation of (artifact or hearthstone).
    pub fn evocation(self, evokable_id: EvokableId) -> EvocationBuilder {
        EvocationBuilder {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            evokable_id,
            resonant: None,
            dissonant: None,
            evocation_tree: HashSet::new(),
            upgrade_charm: None,
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}
