use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::OneHandedArtifactWeapon, mundane::OneHandedMundaneWeapon};

use super::EquippedOneHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(String, OneHandedMundaneWeapon),
    Artifact(String, OneHandedArtifactWeapon),
}

impl<'source> EquippedOneHandedWeaponNoAttunementMemo {
    pub(crate) fn as_ref(&'source self) -> EquippedOneHandedWeaponNoAttunement<'source> {
        match self {
            EquippedOneHandedWeaponNoAttunementMemo::Mundane(name, memo) => {
                EquippedOneHandedWeaponNoAttunement::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedOneHandedWeaponNoAttunementMemo::Artifact(name, memo) => {
                EquippedOneHandedWeaponNoAttunement::Artifact(name.as_str(), memo.as_ref())
            }
        }
    }
}
