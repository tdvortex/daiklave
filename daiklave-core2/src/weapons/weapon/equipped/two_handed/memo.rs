use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeapon, mundane::TwoHandedMundaneWeapon,
};

use super::EquippedTwoHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedTwoHandedWeaponMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeapon, Option<u8>),
}

impl<'source> EquippedTwoHandedWeaponMemo {
    pub fn as_ref(&'source self) -> EquippedTwoHandedWeapon<'source> {
        match self {
            EquippedTwoHandedWeaponMemo::Mundane(name, memo) => {
                EquippedTwoHandedWeapon::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedTwoHandedWeaponMemo::Artifact(name, memo, attunement) => {
                EquippedTwoHandedWeapon::Artifact(name.as_str(), memo.as_ref(), *attunement)
            }
        }
    }
}
