use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::{TwoHandedArtifactWeaponMemo}, mundane::TwoHandedMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeaponMemo, Option<u8>),
}