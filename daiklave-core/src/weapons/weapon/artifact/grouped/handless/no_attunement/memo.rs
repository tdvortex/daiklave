use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::{NaturalArtifactWeaponMemo, WornArtifactWeaponMemo};

use super::HandlessArtifactWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessArtifactWeaponNoAttunementMemo {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo),
}

impl From<&HandlessArtifactWeaponNoAttunement<'_>> for HandlessArtifactWeaponNoAttunementMemo {
    fn from(value: &HandlessArtifactWeaponNoAttunement<'_>) -> Self {
        match value {
            HandlessArtifactWeaponNoAttunement::Natural(view) => Self::Natural(view.into()),
            HandlessArtifactWeaponNoAttunement::Worn(view) => Self::Worn(view.into()),
        }
    }
}