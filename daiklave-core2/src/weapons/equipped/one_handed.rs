use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{OneHandedMundaneWeapon, OneHandedMundaneWeaponMemo}, artifact::{OneHandedArtifactWeapon, OneHandedArtifactWeaponMemo}};

pub(in crate::weapons) enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum EquippedOneHandedWeapon<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>, Option<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum EquippedOneHandedWeaponMemo {
    Mundane(BaseWeaponId, OneHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeaponMemo, Option<u8>),
}

impl<'source> EquippedOneHandedWeaponMemo {
    pub fn as_ref(&'source self) -> EquippedOneHandedWeapon<'source> {
        match self {
            EquippedOneHandedWeaponMemo::Mundane(id, memo) => EquippedOneHandedWeapon::Mundane(*id, memo.as_ref()),
            EquippedOneHandedWeaponMemo::Artifact(id, memo, attunement) => EquippedOneHandedWeapon::Artifact(*id, memo.as_ref(), *attunement)
        }
    }
}