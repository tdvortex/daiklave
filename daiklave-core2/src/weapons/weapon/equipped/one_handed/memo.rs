use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::{OneHandedArtifactWeaponMemo}, mundane::OneHandedMundaneWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponMemo {
    Mundane(String, OneHandedMundaneWeaponMemo),
    Artifact(String, OneHandedArtifactWeaponMemo, Option<u8>),
}