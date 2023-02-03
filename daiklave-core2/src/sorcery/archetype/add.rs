use super::{SorceryArchetypeDetails, SorceryArchetypeName};

/// A mutation to add a sorcery archetype, as part of a sorcerous initiation.
pub struct AddSorceryArchetype {
    pub(crate) name: SorceryArchetypeName,
    pub(crate) archetype: SorceryArchetypeDetails,
}
