use crate::weapons::weapon::equipped::EquipHand;

use super::{
    newtype::{
        NaturalArtifactWeapon, OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
    },
};

/// A magical weapon, like a daiklave or a direlash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactWeapon(pub(crate) String, pub(crate) ArtifactWeaponHandedness);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactWeaponHandedness {
    Natural(NaturalArtifactWeapon),
    Worn(WornArtifactWeapon, bool),
    OneHanded(OneHandedArtifactWeapon, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeapon, bool),
}