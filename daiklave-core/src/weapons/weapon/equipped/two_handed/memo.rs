use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeaponMemo, mundane::TwoHandedMundaneWeapon,
};

use super::EquippedTwoHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeaponMemo, Option<u8>),
}

impl From<&EquippedTwoHandedWeapon<'_>> for EquippedTwoHandedWeaponMemo {
    fn from(value: &EquippedTwoHandedWeapon<'_>) -> Self {
        match value {
            EquippedTwoHandedWeapon::Mundane(name, weapon) => {
                Self::Mundane((*name).into(), weapon.into())
            }
            EquippedTwoHandedWeapon::Artifact(name, weapon, attunement) => {
                Self::Artifact((*name).into(), weapon.into(), *attunement)
            }
        }
    }
}
