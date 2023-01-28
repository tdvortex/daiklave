use crate::armor::armor_item::{mundane::MundaneArmorName, artifact::ArtifactArmorName};

use super::ArmorName;

/// The name of a piece of armor to be added, removed, equipped, or unequipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorNameMutation {
    /// Mundane, non-artifact armor.
    Mundane(MundaneArmorName),
    /// Artifact armor. This is the name for the specific piece of armor (like
    /// "Brilliant Sentinel"), not the generic item name (like "Articulated
    /// Plate (Artifact)").
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