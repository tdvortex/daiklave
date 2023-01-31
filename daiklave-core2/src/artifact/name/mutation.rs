use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::{weapons::weapon::artifact::ArtifactWeaponName, armor::armor_item::artifact::ArtifactArmorName, artifact::wonders::WonderName};

use super::ArtifactName;

/// The name of a magical creation (weapon, armor, warstrider, or wonder).
/// For use in adding, removing, or otherwise changing a character's state with
/// regards to an artifact.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum ArtifactNameMutation {
    /// An artifact weapon's name.
    Weapon(ArtifactWeaponName),
    /// An artifact armor item's name.
    Armor(ArtifactArmorName),
    /// A wonder's name.
    Wonder(WonderName),
}

impl Deref for ArtifactNameMutation {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            ArtifactNameMutation::Weapon(name) => &**name,
            ArtifactNameMutation::Armor(name) => &**name,
            ArtifactNameMutation::Wonder(name) => &**name,
        }
    }
}

impl From<ArtifactName<'_>> for ArtifactNameMutation {
    fn from(name: ArtifactName<'_>) -> Self {
        match name {
            ArtifactName::Weapon(name) => Self::Weapon(name.into()),
            ArtifactName::Armor(name) => Self::Armor(name.into()),
            ArtifactName::Wonder(name) => Self::Wonder(name.into()),
        }
    }
}

impl From<ArtifactWeaponName> for ArtifactNameMutation {
    fn from(artifact_weapon_name: ArtifactWeaponName) -> Self {
        Self::Weapon(artifact_weapon_name)
    }
}

impl From<ArtifactArmorName> for ArtifactNameMutation {
    fn from(artifact_armor_name: ArtifactArmorName) -> Self {
        Self::Armor(artifact_armor_name)
    }
}

impl From<WonderName> for ArtifactNameMutation {
    fn from(wonder_name: WonderName) -> Self {
        Self::Wonder(wonder_name)
    }
}