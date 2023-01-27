use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::TwoHandedArtifactWeapon, mundane::TwoHandedMundaneWeapon,
};

use super::EquippedTwoHandedWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedTwoHandedWeaponNoAttunementMemo {
    Mundane(String, TwoHandedMundaneWeapon),
    Artifact(String, TwoHandedArtifactWeapon),
}

impl<'source> EquippedTwoHandedWeaponNoAttunementMemo {
    pub(crate) fn as_ref(&'source self) -> EquippedTwoHandedWeaponNoAttunement<'source> {
        match self {
            EquippedTwoHandedWeaponNoAttunementMemo::Mundane(name, memo) => {
                EquippedTwoHandedWeaponNoAttunement::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedTwoHandedWeaponNoAttunementMemo::Artifact(name, memo) => {
                EquippedTwoHandedWeaponNoAttunement::Artifact(name.as_str(), memo.as_ref())
            }
        }
    }
}
