use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{
    armor::armor_item::artifact::ArtifactArmorName, artifact::wonders::WonderName,
    weapons::weapon::artifact::ArtifactWeaponName,
};

use super::ArtifactName;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub(crate) enum ArtifactNameMutation {
    Weapon(ArtifactWeaponName),
    Armor(ArtifactArmorName),
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

impl<'source> Into<ArtifactName<'source>> for &'source ArtifactNameMutation {
    fn into(self) -> ArtifactName<'source> {
        match self {
            ArtifactNameMutation::Weapon(name) => ArtifactName::Weapon(name.as_str()),
            ArtifactNameMutation::Armor(name) => ArtifactName::Armor(name.as_str()),
            ArtifactNameMutation::Wonder(name) => ArtifactName::Wonder(name.as_str()),
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
