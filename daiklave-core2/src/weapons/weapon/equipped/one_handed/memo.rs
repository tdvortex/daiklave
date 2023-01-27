use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::OneHandedArtifactWeapon, mundane::OneHandedMundaneWeapon};

use super::EquippedOneHandedWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedOneHandedWeaponMemo {
    Mundane(String, OneHandedMundaneWeapon),
    Artifact(String, OneHandedArtifactWeapon, Option<u8>),
}

impl<'source> EquippedOneHandedWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> EquippedOneHandedWeapon<'source> {
        match self {
            EquippedOneHandedWeaponMemo::Mundane(name, memo) => {
                EquippedOneHandedWeapon::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedOneHandedWeaponMemo::Artifact(name, memo, attunement) => {
                EquippedOneHandedWeapon::Artifact(name.as_str(), memo.as_ref(), *attunement)
            }
        }
    }
}
