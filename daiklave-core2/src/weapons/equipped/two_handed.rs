use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{TwoHandedMundaneWeapon, TwoHandedMundaneWeaponMemo, MundaneWeapon}, artifact::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo, ArtifactWeapon}, WeaponId, Weapon, WeaponType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>),
}

impl<'view, 'source> EquippedTwoHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> EquippedTwoHandedWeaponNoAttunementMemo {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(id, view) => EquippedTwoHandedWeaponNoAttunementMemo::Mundane(*id, view.as_memo()),
            EquippedTwoHandedWeaponNoAttunement::Artifact(id, view) => EquippedTwoHandedWeaponNoAttunementMemo::Artifact(*id, view.as_memo()),
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'view, 'source>> {
        match (weapon_id, self) {
            (WeaponId::Unarmed, _) => Some(crate::weapons::unarmed()),
            (WeaponId::Mundane(target_id), EquippedTwoHandedWeaponNoAttunement::Mundane(actual_id, two)) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(target_id, MundaneWeapon::TwoHanded(*two, true))))
                }
            }
            (WeaponId::Artifact(target_id), EquippedTwoHandedWeaponNoAttunement::Artifact(actual_id, two)) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(target_id, ArtifactWeapon::TwoHanded(two, true), None)))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(base_id, _) => std::iter::once(WeaponId::Mundane(*base_id)),
            EquippedTwoHandedWeaponNoAttunement::Artifact(artifact_id, _) => std::iter::once(WeaponId::Artifact(*artifact_id)),
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

impl<'view, 'source> EquippedTwoHandedWeapon<'source> {
    pub fn as_memo(&'source self) -> EquippedTwoHandedWeaponMemo {
        match self {
            EquippedTwoHandedWeapon::Mundane(id, view) => EquippedTwoHandedWeaponMemo::Mundane(*id, view.as_memo()),
            EquippedTwoHandedWeapon::Artifact(id, view, attunement) => EquippedTwoHandedWeaponMemo::Artifact(*id, view.as_memo(), *attunement)
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'view, 'source>> {
        match (self, weapon_id) {
            (EquippedTwoHandedWeapon::Mundane(actual_id, two), WeaponId::Mundane(target_id)) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(target_id, MundaneWeapon::TwoHanded(*two, true))))
                }
            }
            (EquippedTwoHandedWeapon::Artifact(actual_id, two, attunement), WeaponId::Artifact(target_id)) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(target_id, ArtifactWeapon::TwoHanded(two, true), *attunement)))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            EquippedTwoHandedWeapon::Mundane(base_id, _) => std::iter::once(WeaponId::Mundane(*base_id)),
            EquippedTwoHandedWeapon::Artifact(artifact_id, _, _) => std::iter::once(WeaponId::Artifact(*artifact_id)),
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