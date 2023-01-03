use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{TwoHandedMundaneWeapon, TwoHandedMundaneWeaponMemo}, artifact::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo}};

pub(in crate::weapons) enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum EquippedTwoHandedWeapon<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>, Option<u8>),
}

impl<'source> EquippedTwoHandedWeapon<'source> {
    pub fn as_memo(&'source self) -> EquippedTwoHandedWeaponMemo {
        match self {
            EquippedTwoHandedWeapon::Mundane(id, view) => EquippedTwoHandedWeaponMemo::Mundane(*id, view.as_memo()),
            EquippedTwoHandedWeapon::Artifact(id, view, attunement) => EquippedTwoHandedWeaponMemo::Artifact(*id, view.as_memo(), *attunement)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum EquippedTwoHandedWeaponMemo {
    Mundane(BaseWeaponId, TwoHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeaponMemo, Option<u8>),
}

impl<'source> EquippedTwoHandedWeaponMemo {
    pub fn as_ref(&'source self) -> EquippedTwoHandedWeapon<'source> {
        match self {
            EquippedTwoHandedWeaponMemo::Mundane(id, memo) => EquippedTwoHandedWeapon::Mundane(*id, memo.as_ref()),
            EquippedTwoHandedWeaponMemo::Artifact(id, memo, attunement) => EquippedTwoHandedWeapon::Artifact(*id, memo.as_ref(), *attunement)
        }
    }
}