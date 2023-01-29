use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::TwoHandedArtifactWeapon, mundane::TwoHandedMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeapon, Option<u8>),
}