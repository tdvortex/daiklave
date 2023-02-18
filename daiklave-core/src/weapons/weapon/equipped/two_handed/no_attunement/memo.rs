use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeaponMemo, mundane::TwoHandedMundaneWeapon,
};

use super::EquippedTwoHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponNoAttunementMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeaponMemo),
}

impl From<&EquippedTwoHandedWeaponNoAttunement<'_>> for EquippedTwoHandedWeaponNoAttunementMemo {
    fn from(value: &EquippedTwoHandedWeaponNoAttunement<'_>) -> Self {
        match value {
            EquippedTwoHandedWeaponNoAttunement::Mundane(name, weapon) => Self::Mundane((*name).into(), weapon.into()),
            EquippedTwoHandedWeaponNoAttunement::Artifact(name, weapon) => Self::Artifact((*name).into(), weapon.into()),
        }
    }
}