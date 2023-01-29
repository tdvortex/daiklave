use crate::{sorcery::SorceryArchetypeName, CharacterMutation, merits::merit::AddMerit};

use super::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName, builder::SorceryArchetypeMeritBuilderWithDots};

pub struct AddSorceryArchetypeMerit {
    archetype_name: SorceryArchetypeName,
    merit_name: SorceryArchetypeMeritName,
    merit: SorceryArchetypeMeritDetails,
}

impl From<SorceryArchetypeMeritBuilderWithDots> for AddSorceryArchetypeMerit {
    fn from(builder: SorceryArchetypeMeritBuilderWithDots) -> Self {
        builder.build()
    }
}

impl From<AddSorceryArchetypeMerit> for CharacterMutation {
    fn from(add_sorcery_merit: AddSorceryArchetypeMerit) -> Self {
        AddMerit::from(add_sorcery_merit).into()
    }
}