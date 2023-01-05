use serde::{Deserialize, Serialize};

use crate::weapons::{
    artifact::{ArtifactWeapon, OneHandedArtifactWeapon, OneHandedArtifactWeaponMemo},
    mundane::{MundaneWeapon, OneHandedMundaneWeapon, OneHandedMundaneWeaponMemo},
    ArtifactWeaponId, BaseWeaponId, EquipHand, Weapon, WeaponId, WeaponType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>),
}

impl<'view, 'source> EquippedOneHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&'source self) -> EquippedOneHandedWeaponNoAttunementMemo {
        match self {
            EquippedOneHandedWeaponNoAttunement::Mundane(id, view) => {
                EquippedOneHandedWeaponNoAttunementMemo::Mundane(*id, view.as_memo())
            }
            EquippedOneHandedWeaponNoAttunement::Artifact(id, view) => {
                EquippedOneHandedWeaponNoAttunementMemo::Artifact(*id, view.as_memo())
            }
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        hand: EquipHand,
    ) -> Option<Weapon<'source>> {
        match (weapon_id, self) {
            (WeaponId::Unarmed, _) => Some(crate::weapons::unarmed()),
            (
                WeaponId::Mundane(target_id),
                EquippedOneHandedWeaponNoAttunement::Mundane(actual_id, one),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::OneHanded(*one, Some(hand)),
                    )))
                }
            }
            (
                WeaponId::Artifact(target_id),
                EquippedOneHandedWeaponNoAttunement::Artifact(actual_id, one),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::OneHanded(one, Some(hand)),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> {
        match self {
            EquippedOneHandedWeaponNoAttunement::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedOneHandedWeaponNoAttunement::Artifact(artifact_id, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedOneHandedWeaponNoAttunementMemo {
    Mundane(BaseWeaponId, OneHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeaponMemo),
}

impl<'source> EquippedOneHandedWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> EquippedOneHandedWeaponNoAttunement<'source> {
        match self {
            EquippedOneHandedWeaponNoAttunementMemo::Mundane(id, memo) => {
                EquippedOneHandedWeaponNoAttunement::Mundane(*id, memo.as_ref())
            }
            EquippedOneHandedWeaponNoAttunementMemo::Artifact(id, memo) => {
                EquippedOneHandedWeaponNoAttunement::Artifact(*id, memo.as_ref())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedOneHandedWeapon<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(
        ArtifactWeaponId,
        OneHandedArtifactWeapon<'source>,
        Option<u8>,
    ),
}

impl<'source> From<EquippedOneHandedWeaponNoAttunement<'source>>
    for EquippedOneHandedWeapon<'source>
{
    fn from(unattuned: EquippedOneHandedWeaponNoAttunement<'source>) -> Self {
        match unattuned {
            EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane) => Self::Mundane(id, mundane),
            EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact) => {
                Self::Artifact(id, artifact, None)
            }
        }
    }
}

impl<'view, 'source> EquippedOneHandedWeapon<'source> {
    pub fn as_memo(&self) -> EquippedOneHandedWeaponMemo {
        match self {
            EquippedOneHandedWeapon::Mundane(id, view) => {
                EquippedOneHandedWeaponMemo::Mundane(*id, view.as_memo())
            }
            EquippedOneHandedWeapon::Artifact(id, view, attunement) => {
                EquippedOneHandedWeaponMemo::Artifact(*id, view.as_memo(), *attunement)
            }
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        hand: EquipHand,
    ) -> Option<Weapon<'source>> {
        match (self, weapon_id) {
            (EquippedOneHandedWeapon::Mundane(actual_id, one), WeaponId::Mundane(target_id)) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::OneHanded(*one, Some(hand)),
                    )))
                }
            }
            (
                EquippedOneHandedWeapon::Artifact(actual_id, one, attunement),
                WeaponId::Artifact(target_id),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::OneHanded(one, Some(hand)),
                        *attunement,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> {
        match self {
            EquippedOneHandedWeapon::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedOneHandedWeapon::Artifact(artifact_id, _, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedOneHandedWeaponMemo {
    Mundane(BaseWeaponId, OneHandedMundaneWeaponMemo),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeaponMemo, Option<u8>),
}

impl<'source> EquippedOneHandedWeaponMemo {
    pub fn as_ref(&'source self) -> EquippedOneHandedWeapon<'source> {
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
