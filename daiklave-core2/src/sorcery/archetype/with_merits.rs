use std::collections::HashMap;

use crate::book_reference::BookReference;

use super::{SorceryArchetypeDetails, SorceryArchetypeMeritDetails, SorceryArchetypeMerits};

pub struct SorceryArchetypeWithMerits<'view, 'source> {
    archetype_name: &'source str,
    archetype: &'source SorceryArchetypeDetails,
    merits: &'view HashMap<&'source str, &'source SorceryArchetypeMeritDetails>
}

impl<'view, 'source> SorceryArchetypeWithMerits<'view, 'source> {
    pub fn name(&self) -> &'source str {
        self.archetype_name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.archetype.book_reference
    }

    pub fn description(&self) -> &'source str {
        &self.archetype.description
    }

    pub fn merits(&self) -> SorceryArchetypeMerits<'view, 'source> {
        SorceryArchetypeMerits {
            archetype_name: self.archetype_name,
            merits: self.merits
        }
    }
}

