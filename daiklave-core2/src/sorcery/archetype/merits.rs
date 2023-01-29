use std::collections::HashMap;

use super::{SorceryArchetypeMeritDetails, merit::SorceryArchetypeMerit};

pub struct SorceryArchetypeMerits<'view, 'source> {
    archetype_name: &'source str,
    merits: &'view HashMap<&'source str, &'source SorceryArchetypeMeritDetails>
}

impl<'view, 'source> SorceryArchetypeMerits<'view, 'source> {
    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.merits.keys().copied()
    }

    pub fn get(&self, merit_name: &str) -> Option<SorceryArchetypeMerit<'source>> {
        self.merits.get_key_value(merit_name).map(|(merit_name, merit_details)| SorceryArchetypeMerit {
            archetype_name: self.archetype_name,
            name: *merit_name,
            details: *merit_details,
        })
    }
}