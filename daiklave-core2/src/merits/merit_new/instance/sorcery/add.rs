use crate::{sorcery::SorceryArchetypeName, merits::merit_new::template::builder::{SorceryArchetypeMeritBuilder, SorceryArchetypeMeritBuilderWithDescription}};

use super::{SorceryArchetypeMeritName, details::SorceryArchetypeMeritDetails};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSorceryArchetypeMerit {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) name: SorceryArchetypeMeritName,
    pub(crate) details: SorceryArchetypeMeritDetails,
}

impl AddSorceryArchetypeMerit {
    pub fn archetype_name(archetype_name: impl Into<SorceryArchetypeName>) -> SorceryArchetypeMeritBuilder {
        SorceryArchetypeMeritBuilder::archetype_name(archetype_name)
    }
}

impl From<SorceryArchetypeMeritBuilderWithDescription> for AddSorceryArchetypeMerit {
    fn from(builder: SorceryArchetypeMeritBuilderWithDescription) -> Self {
        builder.build()
    }
}