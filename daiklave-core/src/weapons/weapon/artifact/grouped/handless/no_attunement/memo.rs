use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::{NaturalArtifactWeaponMemo, WornArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessArtifactWeaponNoAttunementMemo {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo),
}
