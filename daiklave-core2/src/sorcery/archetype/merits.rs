use std::collections::HashMap;

use crate::merits::{merit_new::Merit, merit_new::{MeritSource, SorceryArchetypeMerit, SorceryArchetypeMeritDetails}};

pub struct SorceryArchetypeMerits<'view, 'source> {
    archetype_name: &'source str,
    merits: &'view HashMap<&'source str, &'source SorceryArchetypeMeritDetails>
}

impl<'view, 'source> SorceryArchetypeMerits<'view, 'source> {
    pub fn iter(&self) -> impl Iterator<Item = Merit<'source>> + '_ {
        self.merits.iter().map(|(merit_name, details)| {
            Merit(MeritSource::SorceryArchetype(SorceryArchetypeMerit {
                archetype_name: self.archetype_name,
                merit_name,
                details,
            }))
        })
    }
}