use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::TwoHandedArtifactWeaponMemo, mundane::TwoHandedMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponNoAttunementMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeaponMemo),
}