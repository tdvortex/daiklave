use crate::{merits::merit::RemoveMerit, sorcery::SorceryArchetypeName, CharacterMutation};

use super::SorceryArchetypeMeritName;

/// A mutation to remove a sorcery archetype merit from a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveSorceryArchetypeMerit {
    /// The name of the archetype to which the merit belongs.
    pub archetype_name: SorceryArchetypeName,
    /// The name of the merit.
    pub name: SorceryArchetypeMeritName,
}

impl From<RemoveSorceryArchetypeMerit> for CharacterMutation {
    fn from(remove_sorcery_merit: RemoveSorceryArchetypeMerit) -> Self {
        RemoveMerit::Sorcery(remove_sorcery_merit).into()
    }
}
