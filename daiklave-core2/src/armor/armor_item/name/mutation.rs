use crate::armor::armor_item::{artifact::ArtifactArmorName, mundane::MundaneArmorName};

use super::ArmorName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArmorNameMutation {
    Mundane(MundaneArmorName),
    Artifact(ArtifactArmorName),
}

impl From<ArmorName<'_>> for ArmorNameMutation {
    fn from(name: ArmorName) -> Self {
        match name {
            ArmorName::Mundane(name) => Self::Mundane(name.into()),
            ArmorName::Artifact(name) => Self::Artifact(name.into()),
        }
    }
}

impl<'source> Into<ArmorName<'source>> for &'source ArmorNameMutation {
    fn into(self) -> ArmorName<'source> {
        match self {
            ArmorNameMutation::Mundane(name) => ArmorName::Mundane(name.as_str()),
            ArmorNameMutation::Artifact(name) => ArmorName::Artifact(name.as_str()),
        }
    }
}

impl From<MundaneArmorName> for ArmorNameMutation {
    fn from(mundane_armor_name: MundaneArmorName) -> Self {
        Self::Mundane(mundane_armor_name)
    }
}

impl From<ArtifactArmorName> for ArmorNameMutation {
    fn from(artifact_armor_name: ArtifactArmorName) -> Self {
        Self::Artifact(artifact_armor_name)
    }
}
