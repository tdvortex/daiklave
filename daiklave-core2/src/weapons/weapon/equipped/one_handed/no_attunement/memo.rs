use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::OneHandedArtifactWeapon, mundane::OneHandedMundaneWeapon, ArtifactWeaponId,
};

use super::EquippedOneHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(String, OneHandedMundaneWeapon),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon),
}

impl<'source> EquippedOneHandedWeaponNoAttunementMemo {
    pub(crate) fn as_ref(&'source self) -> EquippedOneHandedWeaponNoAttunement<'source> {
        match self {
            EquippedOneHandedWeaponNoAttunementMemo::Mundane(name, memo) => {
                EquippedOneHandedWeaponNoAttunement::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedOneHandedWeaponNoAttunementMemo::Artifact(id, memo) => {
                EquippedOneHandedWeaponNoAttunement::Artifact(*id, memo.as_ref())
            }
        }
    }
}
