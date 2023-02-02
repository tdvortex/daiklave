mod fixed;
mod sorcery;
mod variable;
pub use fixed::FixedMeritTemplateBuilder;
pub use sorcery::{SorceryArchetypeMeritBuilder, SorceryArchetypeMeritBuilderWithDescription, SorceryArchetypeMeritBuilderWithDots, SorceryArchetypeMeritBuilderWithName};
pub use variable::*;

use std::collections::{HashSet};

use crate::{book_reference::BookReference, merits::{merit::{instance::{SorceryArchetypeMeritName}}}, sorcery::SorceryArchetypeName};

pub struct MeritTemplateBuilder {
    name: String,
    book_reference: Option<BookReference>,
}

impl MeritTemplateBuilder {
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            book_reference: None,
        }
    }

    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn fixed_dots(self, dots: u8) -> FixedMeritTemplateBuilder {
        FixedMeritTemplateBuilder {
            name: self.name,
            dots,
            book_reference: self.book_reference,
            prerequisites: HashSet::new(),
        }
    }

    pub fn variable_dots(self) -> VariableMeritTemplateBuilder {
        VariableMeritTemplateBuilder {
            name: self.name,
            prerequisites: HashSet::new(),
            book_reference: self.book_reference,
        }
    }

    pub fn sorcery_archetype(self, archetype_name: impl Into<SorceryArchetypeName>) -> SorceryArchetypeMeritBuilderWithName {
        SorceryArchetypeMeritBuilderWithName {
            name: SorceryArchetypeMeritName::from(self.name),
            archetype_name: archetype_name.into(),
            book_reference: self.book_reference,
        }
    }
}

