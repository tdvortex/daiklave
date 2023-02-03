mod add;
mod name;
use std::collections::HashMap;

pub use add::AddSorceryArchetype;
pub use name::SorceryArchetypeName;

mod shaping_ritual;
pub(crate) use shaping_ritual::ShapingRitualDetails;
pub use shaping_ritual::{AddShapingRitual, ShapingRitual};

mod details;
pub(crate) use details::SorceryArchetypeDetails;

use crate::{
    book_reference::BookReference,
    merits::merit::{Merit, MeritSource, SorceryArchetypeMerit, SorceryArchetypeMeritDetails},
};

/// A sorcery archetype which the character has initiated into.
pub struct SorceryArchetype<'view, 'source> {
    pub(crate) archetype_name: &'source str,
    pub(crate) archetype: &'source SorceryArchetypeDetails,
    pub(crate) merits: &'view HashMap<&'source str, &'source SorceryArchetypeMeritDetails>,
}

impl<'view, 'source> SorceryArchetype<'view, 'source> {
    /// The name of the sorcery archetype.
    pub fn name(&self) -> &'source str {
        self.archetype_name
    }

    /// The book reference for the archetype, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.archetype.book_reference
    }

    /// A description of the archetype.
    pub fn description(&self) -> &'source str {
        &self.archetype.description
    }

    /// The merits associated with this archetype.
    pub fn merits(&self) -> Vec<Merit<'source>> {
        self.merits
            .iter()
            .map(|(&merit_name, details)| {
                Merit(MeritSource::SorceryArchetype(SorceryArchetypeMerit {
                    archetype_name: self.archetype_name,
                    merit_name,
                    details,
                }))
            })
            .collect()
    }
}
