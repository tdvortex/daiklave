use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeapon, mundane::TwoHandedMundaneWeapon, ArtifactWeaponId,
};

use super::EquippedTwoHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon, Option<u8>),
}

impl<'source> EquippedTwoHandedWeaponMemo {
    pub fn as_ref(&'source self) -> EquippedTwoHandedWeapon<'source> {
        match self {
            EquippedTwoHandedWeaponMemo::Mundane(name, memo) => {
                EquippedTwoHandedWeapon::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedTwoHandedWeaponMemo::Artifact(id, memo, attunement) => {
                EquippedTwoHandedWeapon::Artifact(*id, memo.as_ref(), *attunement)
            }
        }
    }
}
