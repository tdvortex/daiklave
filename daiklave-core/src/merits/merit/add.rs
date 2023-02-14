use serde::{Serialize, Deserialize};

use crate::{
    artifact::AddArtifact, hearthstones::hearthstone::AddHearthstone,
    languages::language::AddLanguage, martial_arts::style::AddMartialArtsStyle,
    sorcery::AddTerrestrialSorcery, CharacterMutation,
};

use super::{
    instance::AddDemense, manse::AddManse, template::builder::MeritTemplateBuilder,
    AddNonStackableMerit, AddSorceryArchetypeMerit, AddStackableMerit,
};

/// A mutation to add a merit to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddMerit {
    /// Adds an artifact, which may be a weapon, armor, warstrider, or other
    /// wonder.
    Artifact(AddArtifact),
    /// Adds a standalone demense (without a manse) to a character.
    Demense(AddDemense),
    /// Adds the Exalted Healing merit to a character. All Exalts get this for
    /// free.
    ExaltedHealing,
    /// Adds a standalone hearthstone (without a manse) to a character.
    Hearthstone(AddHearthstone),
    /// Adds a non-native language to the character. Major languages are one
    /// merit dot each; local tongues are four-for-one.
    Language(AddLanguage),
    /// Adds a manse, and its associated demense and hearthstone, to a character.
    Manse(AddManse),
    /// Adds a new Martial Arts style to a character. This conveys only the
    /// Martial Artist merit; purchasing the Martial Arts ability and Martial
    /// Arts Charms must be done afterwards.
    MartialArtist(AddMartialArtsStyle),
    /// Adds the Terrestrial circle of sorcery to a mortal.
    MortalSorcerer(AddTerrestrialSorcery),
    /// Adds a non-stackable, single-purchase merit to a character.
    NonStackable(AddNonStackableMerit),
    /// Adds a merit to a character which is tied to a sorcery archetype the
    /// character possesses as part of their sorcerous initiation.
    Sorcery(AddSorceryArchetypeMerit),
    /// Adds a stackable, multi-purchase merit to a character.
    Stackable(AddStackableMerit),
}

impl AddMerit {
    /// Starts building a new merit by providing its name.
    pub fn name(name: impl Into<String>) -> MeritTemplateBuilder {
        MeritTemplateBuilder::name(name)
    }

    /// Adds the Terrestrial circle of sorcery, but only if the character is a
    /// mortal.
    pub fn mortal_sorcerer(add_terrestrial_sorcery: AddTerrestrialSorcery) -> Self {
        Self::MortalSorcerer(add_terrestrial_sorcery)
    }
}

impl From<AddArtifact> for AddMerit {
    fn from(add_artifact: AddArtifact) -> Self {
        Self::Artifact(add_artifact)
    }
}

impl From<AddDemense> for AddMerit {
    fn from(add_demense: AddDemense) -> Self {
        Self::Demense(add_demense)
    }
}

impl From<AddHearthstone> for AddMerit {
    fn from(add_hearthstone: AddHearthstone) -> Self {
        Self::Hearthstone(add_hearthstone)
    }
}

impl From<AddLanguage> for AddMerit {
    fn from(add_language: AddLanguage) -> Self {
        Self::Language(add_language)
    }
}

impl From<AddManse> for AddMerit {
    fn from(add_manse: AddManse) -> Self {
        Self::Manse(add_manse)
    }
}

impl From<AddMartialArtsStyle> for AddMerit {
    fn from(add_martial_arts_style: AddMartialArtsStyle) -> Self {
        Self::MartialArtist(add_martial_arts_style)
    }
}

// Deliberately NOT implementing From<AddTerrestrialSorcery>.

impl From<AddNonStackableMerit> for AddMerit {
    fn from(add_non_stackable_merit: AddNonStackableMerit) -> Self {
        Self::NonStackable(add_non_stackable_merit)
    }
}

impl From<AddSorceryArchetypeMerit> for AddMerit {
    fn from(add_sorcery_archetype_merit: AddSorceryArchetypeMerit) -> Self {
        Self::Sorcery(add_sorcery_archetype_merit)
    }
}

impl From<AddStackableMerit> for AddMerit {
    fn from(add_stackable_merit: AddStackableMerit) -> Self {
        Self::Stackable(add_stackable_merit)
    }
}

impl From<AddMerit> for CharacterMutation {
    fn from(add_merit: AddMerit) -> Self {
        CharacterMutation::AddMerit(add_merit)
    }
}
