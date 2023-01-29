use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::newtype::{
    OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeapon),
    OneHanded(OneHandedArtifactWeapon),
    TwoHanded(TwoHandedArtifactWeapon),
}
