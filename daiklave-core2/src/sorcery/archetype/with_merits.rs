use std::collections::HashMap;

use super::{SorceryArchetype, SorceryArchetypeMerit};

/// A sorcery archetype owned by a character, and a hashmap of all merits for
/// that archetype which the character possesses.
pub type SorceryArchetypeWithMerits<'view, 'source> = (
    &'source str, // Sorcery archetype name
    &'source SorceryArchetype,
    &'view HashMap<&'source str, &'source SorceryArchetypeMerit>,
);
