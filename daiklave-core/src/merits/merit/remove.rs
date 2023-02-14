use serde::{Serialize, Deserialize};

use crate::{
    artifact::RemoveArtifact, hearthstones::hearthstone::HearthstoneName,
    languages::language::RemoveLanguage, martial_arts::style::MartialArtsStyleName,
    CharacterMutation,
};

use super::{
    manse::ManseName, DemenseName, RemoveNonStackableMerit, RemoveSorceryArchetypeMerit,
    RemoveStackableMerit,
};

/// A mutation to remove a merit from the character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoveMerit {
    /// Removes a specific artifact from the character.
    Artifact(RemoveArtifact),
    /// Removes a demense from the character. If the demense has a manse, the
    /// manse and its hearthstone will also be removed.
    Demense(DemenseName),
    /// Removes the Exalted Healing merit from a character; this cannot be
    /// applied to Exalts (who always have it).
    ExaltedHealing,
    /// Removes a hearthstone from a character. If the hearthstone has a manse,
    /// the manse and its demense will also be removed.
    Hearthstone(HearthstoneName),
    /// Removes a language from the character. Native languages cannot be
    /// removed.
    Language(RemoveLanguage),
    /// Removes a manse, its associated demense, and its associated hearthstone
    /// from a character.
    Manse(ManseName),
    /// Removes a martial arts style, all its martial arts dots, and any
    /// Martial Arts Charms from the character.
    MartialArtist(MartialArtsStyleName),
    /// Removes the character's sorcery, if they are mortal.
    MortalSorcerer,
    /// Removes a non-stackable merit from the character.
    NonStackable(RemoveNonStackableMerit),
    /// Removes a merit from a sorcery archetype the character possesses.
    Sorcery(RemoveSorceryArchetypeMerit),
    /// Removes a specific instance of a stackable merit.
    Stackable(RemoveStackableMerit),
}

impl From<RemoveArtifact> for RemoveMerit {
    fn from(remove_artifact: RemoveArtifact) -> Self {
        Self::Artifact(remove_artifact)
    }
}

impl From<RemoveLanguage> for RemoveMerit {
    fn from(remove_language: RemoveLanguage) -> Self {
        Self::Language(remove_language)
    }
}

impl From<RemoveMerit> for CharacterMutation {
    fn from(remove_merit: RemoveMerit) -> Self {
        Self::RemoveMerit(remove_merit)
    }
}
