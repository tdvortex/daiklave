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
    Weapon, WeaponName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedTwoHandedWeapon<'source> {
    Mundane(&'source str, TwoHandedMundaneWeaponView<'source>),
    Artifact(
        &'source str,
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
    pub fn get_weapon(&'view self, name: WeaponName<'_>) -> Option<Weapon<'source>> {
        match (self, name) {
            (EquippedTwoHandedWeapon::Mundane(name, two), WeaponName::Mundane(target_name)) => {
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
                EquippedTwoHandedWeapon::Artifact(name, two, attunement),
                WeaponName::Artifact(target_name),
            ) => {
                if &target_name != name {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        *name,
                        ArtifactWeaponView::TwoHanded(two.clone(), true),
                        *attunement,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponName<'source>> + '_ {
        match self {
            EquippedTwoHandedWeapon::Mundane(name, _) => {
                std::iter::once(WeaponName::Mundane(*name))
            }
            EquippedTwoHandedWeapon::Artifact(name, _, _) => {
                std::iter::once(WeaponName::Artifact(*name))
            }
        }
    }
}
