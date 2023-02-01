use crate::{artifact::AddArtifact, languages::language::AddLanguage, hearthstones::hearthstone::AddHearthstone, martial_arts::style::AddMartialArtsStyle, sorcery::AddTerrestrialSorcery, CharacterMutation};

use super::{AddStackableMerit, AddNonStackableMerit, instance::AddDemense, AddSorceryArchetypeMerit, manse::AddManse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddMerit {
    Artifact(AddArtifact),
    Demense(AddDemense),
    ExaltedHealing,
    Hearthstone(AddHearthstone),
    Language(AddLanguage),
    Manse(AddManse),
    MartialArtist(AddMartialArtsStyle),
    MortalSorcerer(AddTerrestrialSorcery),
    NonStackable(AddNonStackableMerit),
    Sorcery(AddSorceryArchetypeMerit),
    Stackable(AddStackableMerit),
}

impl AddMerit {
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