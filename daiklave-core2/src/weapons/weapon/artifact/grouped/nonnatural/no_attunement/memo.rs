use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::{WornArtifactWeaponMemo, OneHandedArtifactWeaponMemo, TwoHandedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeaponMemo),
    OneHanded(OneHandedArtifactWeaponMemo),
    TwoHanded(TwoHandedArtifactWeaponMemo),
}
