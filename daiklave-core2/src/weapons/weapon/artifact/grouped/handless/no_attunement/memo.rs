use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::newtype::{NaturalArtifactWeapon, WornArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessArtifactWeaponNoAttunementMemo {
    Natural(NaturalArtifactWeapon),
    Worn(WornArtifactWeapon),
}