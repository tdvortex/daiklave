use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::{
    OneHandedArtifactWeaponMemo, TwoHandedArtifactWeaponMemo, WornArtifactWeaponMemo,
};

use super::NonnaturalArtifactWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeaponMemo),
    OneHanded(OneHandedArtifactWeaponMemo),
    TwoHanded(TwoHandedArtifactWeaponMemo),
}

impl From<&NonnaturalArtifactWeaponNoAttunement<'_>> for NonnaturalArtifactWeaponNoAttunementMemo {
    fn from(value: &NonnaturalArtifactWeaponNoAttunement<'_>) -> Self {
        match value {
            NonnaturalArtifactWeaponNoAttunement::Worn(weapon) => Self::Worn(weapon.into()),
            NonnaturalArtifactWeaponNoAttunement::OneHanded(weapon) => Self::OneHanded(weapon.into()),
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(weapon) => Self::TwoHanded(weapon.into()),
        }
    }
}