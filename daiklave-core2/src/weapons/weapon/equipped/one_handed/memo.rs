use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::OneHandedArtifactWeapon, mundane::OneHandedMundaneWeapon, ArtifactWeaponId,
    BaseWeaponId,
};

use super::EquippedOneHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponMemo {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon, Option<u8>),
}

impl<'source> EquippedOneHandedWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> EquippedOneHandedWeapon<'source> {
        match self {
            EquippedOneHandedWeaponMemo::Mundane(id, memo) => {
                EquippedOneHandedWeapon::Mundane(*id, memo.as_ref())
            }
            EquippedOneHandedWeaponMemo::Artifact(id, memo, attunement) => {
                EquippedOneHandedWeapon::Artifact(*id, memo.as_ref(), *attunement)
            }
        }
    }
}
