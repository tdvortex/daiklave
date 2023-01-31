mod demense;
mod exalted_healing;
mod manse;
mod nonstackable;
mod stackable;
pub use demense::AddDemense;
pub use exalted_healing::AddExaltedHealing;
pub use nonstackable::AddNonStackableMerit;
pub use stackable::AddStackableMerit;

use crate::{artifact::AddArtifact, hearthstones::hearthstone::{AddHearthstone}, languages::{language::AddLanguage}, martial_arts::style::AddMartialArtsStyle, CharacterMutation, sorcery::AddSorceryArchetypeMerit};

use self::manse::AddManse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddMerit {
    Artifact(AddArtifact),
    Demense(AddDemense),
    ExaltedHealing,
    Hearthstone(AddHearthstone),
    Language(AddLanguage),
    Manse(AddManse),
    MartialArtist(AddMartialArtsStyle),
    NonStackable(AddNonStackableMerit),
    SorceryArchetype(AddSorceryArchetypeMerit),
    Stackable(AddStackableMerit),
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

impl From<AddExaltedHealing> for AddMerit {
    fn from(add_exalted_healing: AddExaltedHealing) -> Self {
        Self::ExaltedHealing
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

impl From<AddNonStackableMerit> for AddMerit {
    fn from(add_nonstackable: AddNonStackableMerit) -> Self {
        Self::NonStackable(add_nonstackable)
    }
}

impl From<AddSorceryArchetypeMerit> for AddMerit {
    fn from(add_sorcery_archetype_merit: AddSorceryArchetypeMerit) -> Self {
        Self::SorceryArchetype(add_sorcery_archetype_merit)
    }
}

impl From<AddStackableMerit> for AddMerit {
    fn from(add_stackable: AddStackableMerit) -> Self {
        Self::Stackable(add_stackable)
    }
}

impl From<AddMerit> for CharacterMutation {
    fn from(add_merit: AddMerit) -> Self {
        Self::AddMerit(add_merit)
    }
}