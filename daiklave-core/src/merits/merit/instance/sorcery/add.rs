use serde::{Serialize, Deserialize};

use crate::{
    merits::merit::template::builder::{
        SorceryArchetypeMeritBuilder, SorceryArchetypeMeritBuilderWithDescription,
    },
    sorcery::SorceryArchetypeName,
};

use super::{details::SorceryArchetypeMeritDetails, SorceryArchetypeMeritName};

/// A mutation to add a sorcery archetype merit to the character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddSorceryArchetypeMerit {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) name: SorceryArchetypeMeritName,
    pub(crate) details: SorceryArchetypeMeritDetails,
}

impl AddSorceryArchetypeMerit {
    /// Starts a builder to construct a sorcery archetyp merit for the given archetype name.
    pub fn archetype_name(
        archetype_name: impl Into<SorceryArchetypeName>,
    ) -> SorceryArchetypeMeritBuilder {
        SorceryArchetypeMeritBuilder::archetype_name(archetype_name)
    }
}

impl From<SorceryArchetypeMeritBuilderWithDescription> for AddSorceryArchetypeMerit {
    fn from(builder: SorceryArchetypeMeritBuilderWithDescription) -> Self {
        builder.build()
    }
}
