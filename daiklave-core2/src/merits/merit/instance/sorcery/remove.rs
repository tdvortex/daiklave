use crate::{sorcery::SorceryArchetypeName, merits::merit::{RemoveMerit}, CharacterMutation};

use super::SorceryArchetypeMeritName;

/// A mutation to remove a sorcery archetype merit from a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveSorceryArchetypeMerit {
    archetype_name: SorceryArchetypeName,
    name: SorceryArchetypeMeritName,
}

impl From<RemoveSorceryArchetypeMerit> for CharacterMutation {
    fn from(remove_sorcery_merit: RemoveSorceryArchetypeMerit) -> Self {
        RemoveMerit::Sorcery(remove_sorcery_merit).into()
    }
}