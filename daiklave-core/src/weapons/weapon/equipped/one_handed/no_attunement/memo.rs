use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::OneHandedArtifactWeaponMemo, mundane::OneHandedMundaneWeaponMemo,
};

use super::EquippedOneHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(String, OneHandedMundaneWeaponMemo),
    Artifact(String, OneHandedArtifactWeaponMemo),
}

impl From<&EquippedOneHandedWeaponNoAttunement<'_>> for EquippedOneHandedWeaponNoAttunementMemo{
    fn from(value: &EquippedOneHandedWeaponNoAttunement<'_>) -> Self {
        match value {
            EquippedOneHandedWeaponNoAttunement::Mundane(name, view) => Self::Mundane((*name).into(), view.into()),
            EquippedOneHandedWeaponNoAttunement::Artifact(name, view) => Self::Artifact((*name).into(), view.into()),
        }
    }
}