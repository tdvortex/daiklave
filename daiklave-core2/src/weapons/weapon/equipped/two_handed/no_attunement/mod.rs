use crate::weapons::weapon::{
    artifact::{ArtifactWeaponView, TwoHandedArtifactWeapon},
    mundane::{MundaneWeapon, TwoHandedMundaneWeapon},
    weapon_type::WeaponType,
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId,
};

mod memo;
pub use memo::EquippedTwoHandedWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>),
}

impl<'view, 'source> EquippedTwoHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> EquippedTwoHandedWeaponNoAttunementMemo {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(id, view) => {
                EquippedTwoHandedWeaponNoAttunementMemo::Mundane(*id, view.as_memo())
            }
            EquippedTwoHandedWeaponNoAttunement::Artifact(id, view) => {
                EquippedTwoHandedWeaponNoAttunementMemo::Artifact(*id, view.as_memo())
            }
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match (weapon_id, self) {
            (WeaponId::Unarmed, _) => Some(crate::weapons::unarmed()),
            (
                WeaponId::Mundane(target_id),
                EquippedTwoHandedWeaponNoAttunement::Mundane(actual_id, two),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::TwoHanded(two.clone(), true),
                        1,
                    )))
                }
            }
            (
                WeaponId::Artifact(target_id),
                EquippedTwoHandedWeaponNoAttunement::Artifact(actual_id, two),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeaponView::TwoHanded(two.clone(), true),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedTwoHandedWeaponNoAttunement::Artifact(artifact_id, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}
