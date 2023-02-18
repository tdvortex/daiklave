use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::OneHandedArtifactWeaponMemo, mundane::OneHandedMundaneWeaponMemo,
};

use super::EquippedOneHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedOneHandedWeaponMemo {
    Mundane(String, OneHandedMundaneWeaponMemo),
    Artifact(String, OneHandedArtifactWeaponMemo, Option<u8>),
}

impl From<&EquippedOneHandedWeapon<'_>> for EquippedOneHandedWeaponMemo {
    fn from(value: &EquippedOneHandedWeapon<'_>) -> Self {
        match value {
            EquippedOneHandedWeapon::Mundane(name, weapon) => {
                Self::Mundane((*name).into(), weapon.into())
            }
            EquippedOneHandedWeapon::Artifact(name, weapon, attunement) => {
                Self::Artifact((*name).into(), weapon.into(), *attunement)
            }
        }
    }
}
