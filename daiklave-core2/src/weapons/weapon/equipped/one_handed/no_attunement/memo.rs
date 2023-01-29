use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::OneHandedArtifactWeapon, mundane::OneHandedMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(String, OneHandedMundaneWeapon),
    Artifact(String, OneHandedArtifactWeapon),
}
