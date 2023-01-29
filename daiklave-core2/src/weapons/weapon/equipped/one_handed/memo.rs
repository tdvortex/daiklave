use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::OneHandedArtifactWeapon, mundane::OneHandedMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponMemo {
    Mundane(String, OneHandedMundaneWeapon),
    Artifact(String, OneHandedArtifactWeapon, Option<u8>),
}