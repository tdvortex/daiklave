use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{TwoHandedMundaneWeapon, TwoHandedMundaneWeaponMemo}, artifact::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>),
}

impl<'source> EquippedTwoHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> EquippedTwoHandedWeaponNoAttunementMemo {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(id, view) => EquippedTwoHandedWeaponNoAttunementMemo::Mundane(*id, view.as_memo()),
            EquippedTwoHandedWeaponNoAttunement::Artifact(id, view) => EquippedTwoHandedWeaponNoAttunementMemo::Artifact(*id, view.as_memo()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum EquippedTwoHandedWeaponNoAttunementMemo {
    Mundane(BaseWeaponId, TwoHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeaponMemo),
}

impl<'source> EquippedTwoHandedWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> EquippedTwoHandedWeaponNoAttunement<'source> {
        match self {
            EquippedTwoHandedWeaponNoAttunementMemo::Mundane(id, memo) => EquippedTwoHandedWeaponNoAttunement::Mundane(*id, memo.as_ref()),
            EquippedTwoHandedWeaponNoAttunementMemo::Artifact(id, memo) => EquippedTwoHandedWeaponNoAttunement::Artifact(*id, memo.as_ref()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum EquippedTwoHandedWeapon<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>, Option<u8>),
}

impl<'source> From<EquippedTwoHandedWeaponNoAttunement<'source>> for EquippedTwoHandedWeapon<'source> {
    fn from(unattuned: EquippedTwoHandedWeaponNoAttunement<'source>) -> Self {
        match unattuned {
            EquippedTwoHandedWeaponNoAttunement::Mundane(id, mundane) => Self::Mundane(id, mundane),
            EquippedTwoHandedWeaponNoAttunement::Artifact(id, artifact) => Self::Artifact(id, artifact, None),
        }
    }
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