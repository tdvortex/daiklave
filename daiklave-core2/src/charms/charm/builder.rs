use std::collections::{HashMap, HashSet};

use crate::{book_reference::BookReference, sorcery::spell::builder::SpellBuilder};

use super::{
    evocation::{builder::EvocationBuilder, EvokableName},
    spirit::builder::SpiritCharmBuilder,
};

/// Builder for constructing a new Charm (or spell).
pub struct CharmBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
}

impl CharmBuilder {
    /// Starts constructing a new Charm/Spell with the given name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            book_reference: None,
            summary: None,
        }
    }

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
            name: self.name.into(),
            book_reference: self.book_reference,
            summary: self.summary,
            keywords: HashSet::new(),
            control_spell_description: None,
            distortion: None,
        }
    }

    /// Constructs the Charm as an Evocation. Requires specifying what it is an
    /// evocation of (artifact or hearthstone).
    pub fn evocation(self, evokable_name: EvokableName<'_>) -> EvocationBuilder {
        let mut builder = EvocationBuilder::evocation_of(evokable_name);
        if let Some(book_reference) = self.book_reference {
            builder = builder.book_reference(book_reference);
        }

        if let Some(summary) = self.summary {
            builder = builder.summary(summary);
        }

        builder
    }

    /// Constructs the charm as a Spirit charm, which may or may not also be
    /// an Eclipse Charm.
    pub fn spirit(self) -> SpiritCharmBuilder {
        SpiritCharmBuilder {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}
