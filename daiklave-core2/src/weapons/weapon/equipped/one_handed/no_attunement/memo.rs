use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::OneHandedArtifactWeaponMemo, mundane::OneHandedMundaneWeaponMemo, ArtifactWeaponId,
    BaseWeaponId,
};

use super::EquippedOneHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(BaseWeaponId, OneHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeaponMemo),
}

impl<'source> EquippedOneHandedWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> EquippedOneHandedWeaponNoAttunement<'source> {
        match self {
            EquippedOneHandedWeaponNoAttunementMemo::Mundane(id, memo) => {
                EquippedOneHandedWeaponNoAttunement::Mundane(*id, memo.as_ref())
            }
            EquippedOneHandedWeaponNoAttunementMemo::Artifact(id, memo) => {
                EquippedOneHandedWeaponNoAttunement::Artifact(*id, memo.as_ref())
            }
        }
    }
}
