mod fixed;
mod sorcery;
mod variable;
pub use fixed::*;
pub use sorcery::*;
pub use variable::*;

use std::collections::{HashSet};

use crate::{book_reference::BookReference, merits::{merit::{instance::{SorceryArchetypeMeritName}}}, sorcery::SorceryArchetypeName};

/// A builder to construct a new merit template.
pub struct MeritTemplateBuilder {
    name: String,
    book_reference: Option<BookReference>,
}

impl MeritTemplateBuilder {
    /// Starts the builder by providing the merit's name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            book_reference: None,
        }
    }

    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the merit to be purchasable only at the given dot level.
    pub fn fixed_dots(self, dots: u8) -> FixedMeritTemplateBuilder {
        FixedMeritTemplateBuilder {
            name: self.name,
            dots,
            book_reference: self.book_reference,
            prerequisites: HashSet::new(),
        }
    }

    /// Sets the merit to be purchasable at multiple dot levels.
    pub fn variable_dots(self) -> VariableMeritTemplateBuilder {
        VariableMeritTemplateBuilder {
            name: self.name,
            prerequisites: HashSet::new(),
            book_reference: self.book_reference,
        }
    }

    /// Makes the merit dependent on a sorcery archetype.
    pub fn sorcery_archetype(self, archetype_name: impl Into<SorceryArchetypeName>) -> SorceryArchetypeMeritBuilderWithName {
        SorceryArchetypeMeritBuilderWithName {
            name: SorceryArchetypeMeritName::from(self.name),
            archetype_name: archetype_name.into(),
            book_reference: self.book_reference,
        }
    }
}

