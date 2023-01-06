mod memo;
mod no_attunement;
pub use memo::EquippedTwoHandedWeaponMemo;
pub use no_attunement::{
    EquippedTwoHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunementMemo,
};

use crate::weapons::weapon::{
    artifact::{ArtifactWeapon, TwoHandedArtifactWeapon},
    mundane::{MundaneWeapon, TwoHandedMundaneWeapon},
    weapon_type::WeaponType,
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquippedTwoHandedWeapon<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(
        ArtifactWeaponId,
        TwoHandedArtifactWeapon<'source>,
        Option<u8>,
    ),
}

impl<'source> From<EquippedTwoHandedWeaponNoAttunement<'source>>
    for EquippedTwoHandedWeapon<'source>
{
    fn from(unattuned: EquippedTwoHandedWeaponNoAttunement<'source>) -> Self {
        match unattuned {
            EquippedTwoHandedWeaponNoAttunement::Mundane(id, mundane) => Self::Mundane(id, mundane),
            EquippedTwoHandedWeaponNoAttunement::Artifact(id, artifact) => {
                Self::Artifact(id, artifact, None)
            }
        }
    }
}

impl<'view, 'source> EquippedTwoHandedWeapon<'source> {
    pub fn as_memo(&'source self) -> EquippedTwoHandedWeaponMemo {
        match self {
            EquippedTwoHandedWeapon::Mundane(id, view) => {
                EquippedTwoHandedWeaponMemo::Mundane(*id, view.as_memo())
            }
            EquippedTwoHandedWeapon::Artifact(id, view, attunement) => {
                EquippedTwoHandedWeaponMemo::Artifact(*id, view.as_memo(), *attunement)
            }
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match (self, weapon_id) {
            (EquippedTwoHandedWeapon::Mundane(actual_id, two), WeaponId::Mundane(target_id)) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::TwoHanded(two.clone(), true),
                    )))
                }
            }
            (
                EquippedTwoHandedWeapon::Artifact(actual_id, two, attunement),
                WeaponId::Artifact(target_id),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::TwoHanded(two.clone(), true),
                        *attunement,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            EquippedTwoHandedWeapon::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedTwoHandedWeapon::Artifact(artifact_id, _, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}
