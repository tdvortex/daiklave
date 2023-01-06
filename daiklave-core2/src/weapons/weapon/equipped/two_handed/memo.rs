use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeaponMemo, mundane::TwoHandedMundaneWeaponMemo, ArtifactWeaponId,
    BaseWeaponId,
};

use super::EquippedTwoHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedTwoHandedWeaponMemo {
    Mundane(BaseWeaponId, TwoHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeaponMemo, Option<u8>),
}

impl<'source> EquippedTwoHandedWeaponMemo {
    pub fn as_ref(&'source self) -> EquippedTwoHandedWeapon<'source> {
        match self {
            EquippedTwoHandedWeaponMemo::Mundane(id, memo) => {
                EquippedTwoHandedWeapon::Mundane(*id, memo.as_ref())
            }
            EquippedTwoHandedWeaponMemo::Artifact(id, memo, attunement) => {
                EquippedTwoHandedWeapon::Artifact(*id, memo.as_ref(), *attunement)
            }
        }
    }
}
