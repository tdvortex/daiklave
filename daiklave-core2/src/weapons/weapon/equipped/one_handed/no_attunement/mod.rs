use std::num::NonZeroU8;

use crate::weapons::weapon::{
    artifact::{ArtifactWeapon, OneHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, OneHandedMundaneWeaponView},
    name::WeaponName,
    Weapon, WeaponType,
};

pub use memo::EquippedOneHandedWeaponNoAttunementMemo;

use super::EquipHand;

mod memo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(&'source str, OneHandedMundaneWeaponView<'source>),
    Artifact(&'source str, OneHandedArtifactWeaponView<'source>),
}

impl<'view, 'source> EquippedOneHandedWeaponNoAttunement<'source> {
    pub fn get_weapon(
        &'view self,
        name: WeaponName<'_>,
        hand: EquipHand,
    ) -> Option<Weapon<'source>> {
        match (name, self) {
            (WeaponName::Unarmed, _) => Some(crate::weapons::unarmed()),
            (
                WeaponName::Mundane(name),
                EquippedOneHandedWeaponNoAttunement::Mundane(actual_name, one),
            ) => {
                if &name != actual_name {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        *actual_name,
                        MundaneWeaponView::OneHanded(one.clone(), Some(hand)),
                        NonZeroU8::new(1).unwrap(),
                    )))
                }
            }
            (
                WeaponName::Artifact(name),
                EquippedOneHandedWeaponNoAttunement::Artifact(actual_name, one),
            ) => {
                if &name != actual_name {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        *actual_name,
                        ArtifactWeapon::OneHanded(one.clone(), Some(hand)),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponName<'source>> {
        match self {
            EquippedOneHandedWeaponNoAttunement::Mundane(name, _) => {
                std::iter::once(WeaponName::Mundane(*name))
            }
            EquippedOneHandedWeaponNoAttunement::Artifact(name, _) => {
                std::iter::once(WeaponName::Artifact(*name))
            }
        }
    }
}
