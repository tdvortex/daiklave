mod demense;
mod exalted_healing;
mod manse;
mod nonstackable;
pub use demense::AddDemense;
pub use exalted_healing::AddExaltedHealing;
pub use nonstackable::AddNonStackableMerit;

use crate::{artifact::AddArtifact, hearthstones::hearthstone::{AddHearthstone}, sorcery::AddSorcery, languages::AddLanguages, martial_arts::style::AddMartialArtsStyle};

use self::manse::AddManse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddMerit {
    Artifact(AddArtifact),
    Demense(AddDemense),
    ExaltedHealing,
    Hearthstone(AddHearthstone),
    Languages(AddLanguages),
    Manse(AddManse),
    MartialArtist(AddMartialArtsStyle),
    MortalSorcery(AddSorcery),
    NonStackable(AddNonStackableMerit),
    SorceryArchetypeMerit(AddSorceryArchetypeMerit),
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

impl From<AddLanguages> for AddMerit {
    fn from(add_languages: AddLanguages) -> Self {
        Self::Languages(add_languages)
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