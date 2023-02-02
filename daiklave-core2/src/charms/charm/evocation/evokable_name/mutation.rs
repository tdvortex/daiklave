use serde::{Deserialize, Serialize};

use crate::{
    armor::armor_item::artifact::ArtifactArmorName,
    artifact::{wonders::WonderName, ArtifactNameMutation},
    hearthstones::hearthstone::HearthstoneName,
    weapons::weapon::artifact::ArtifactWeaponName,
};

use super::EvokableName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum EvokableNameMutation {
    Hearthstone(HearthstoneName),
    Artifact(ArtifactNameMutation),
}

impl From<EvokableName<'_>> for EvokableNameMutation {
    fn from(name: EvokableName<'_>) -> Self {
        match name {
            EvokableName::Hearthstone(name) => Self::Hearthstone(name.into()),
            EvokableName::Artifact(name) => Self::Artifact(name.into()),
        }
    }
}

impl From<HearthstoneName> for EvokableNameMutation {
    fn from(name: HearthstoneName) -> Self {
        Self::Hearthstone(name.into())
    }
}

impl From<ArtifactNameMutation> for EvokableNameMutation {
    fn from(name: ArtifactNameMutation) -> Self {
        Self::Artifact(name.into())
    }
}

impl From<ArtifactArmorName> for EvokableNameMutation {
    fn from(name: ArtifactArmorName) -> Self {
        Self::Artifact(name.into())
    }
}

impl From<ArtifactWeaponName> for EvokableNameMutation {
    fn from(name: ArtifactWeaponName) -> Self {
        Self::Artifact(name.into())
    }
}

impl From<WonderName> for EvokableNameMutation {
    fn from(name: WonderName) -> Self {
        Self::Artifact(name.into())
    }
}
