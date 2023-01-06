use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeaponMemo, mundane::TwoHandedMundaneWeaponMemo, ArtifactWeaponId,
    BaseWeaponId,
};

use super::EquippedTwoHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedTwoHandedWeaponNoAttunementMemo {
    Mundane(BaseWeaponId, TwoHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeaponMemo),
}

impl<'source> EquippedTwoHandedWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> EquippedTwoHandedWeaponNoAttunement<'source> {
        match self {
            EquippedTwoHandedWeaponNoAttunementMemo::Mundane(id, memo) => {
                EquippedTwoHandedWeaponNoAttunement::Mundane(*id, memo.as_ref())
            }
            EquippedTwoHandedWeaponNoAttunementMemo::Artifact(id, memo) => {
                EquippedTwoHandedWeaponNoAttunement::Artifact(*id, memo.as_ref())
            }
        }
    }
}
