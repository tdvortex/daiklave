use serde::{Serialize, Deserialize};

use crate::weapons::weapon::{mundane::MundaneWeaponName, artifact::ArtifactWeaponName};

use super::WeaponName;

/// The name of a weapon to be added, removed, equipped, or unequipped.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum WeaponNameMutation {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(MundaneWeaponName),
    /// A unique magical weapon.
    Artifact(ArtifactWeaponName),
}

impl From<WeaponName<'_>> for WeaponNameMutation {
    fn from(name: WeaponName) -> Self {
        match name {
            WeaponName::Unarmed => todo!(),
            WeaponName::Mundane(name) => Self::Mundane(name.into()),
            WeaponName::Artifact(name) => Self::Artifact(name.into()),
        }
    }
}

impl From<MundaneWeaponName> for WeaponNameMutation {
    fn from(mundane_name: MundaneWeaponName) -> Self {
        Self::Mundane(mundane_name)
    }
}

impl From<ArtifactWeaponName> for WeaponNameMutation {
    fn from(artifact_name: ArtifactWeaponName) -> Self {
        Self::Artifact(artifact_name)
    }
}