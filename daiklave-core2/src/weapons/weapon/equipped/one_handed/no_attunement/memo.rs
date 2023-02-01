use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::{OneHandedArtifactWeaponMemo}, mundane::OneHandedMundaneWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(String, OneHandedMundaneWeaponMemo),
    Artifact(String, OneHandedArtifactWeaponMemo),
}
