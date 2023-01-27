mod memo;
mod no_attunement;
use std::num::NonZeroU8;

pub(crate) use memo::EquippedTwoHandedWeaponMemo;
pub(crate) use no_attunement::{
    EquippedTwoHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunementMemo,
};

use crate::weapons::weapon::{
    artifact::{ArtifactWeaponView, TwoHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, TwoHandedMundaneWeaponView},
    weapon_type::WeaponType,
    ArtifactWeaponId, Weapon, WeaponId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedTwoHandedWeapon<'source> {
    Mundane(&'source str, TwoHandedMundaneWeaponView<'source>),
    Artifact(
        ArtifactWeaponId,
        TwoHandedArtifactWeaponView<'source>,
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
            EquippedTwoHandedWeapon::Mundane(name, view) => {
                EquippedTwoHandedWeaponMemo::Mundane((*name).to_owned(), view.as_memo())
            }
            EquippedTwoHandedWeapon::Artifact(id, view, attunement) => {
                EquippedTwoHandedWeaponMemo::Artifact(*id, view.as_memo(), *attunement)
            }
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match (self, weapon_id) {
            (EquippedTwoHandedWeapon::Mundane(name, two), WeaponId::Mundane(target_name)) => {
                if &target_name != name {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::TwoHanded(two.clone(), true),
                        NonZeroU8::new(1).unwrap(),
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
                        ArtifactWeaponView::TwoHanded(two.clone(), true),
                        *attunement,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId<'source>> + '_ {
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
