use crate::weapons::weapon::builder::base::BaseWeaponBuilderWithAttack;

use super::{BaseArtifactWeapon, ArtifactWeaponName, builder::{ArtifactWeaponBuilderWithName, ArtifactWeaponBuilder}};

pub struct AddBaseArtifactWeapon {
    pub(crate) name: String,
    pub(crate) weapon: BaseArtifactWeapon
}

impl From<BaseWeaponBuilderWithAttack> for AddBaseArtifactWeapon {
    fn from(builder: BaseWeaponBuilderWithAttack) -> Self {
        builder.build_artifact()
    }
}

impl AddBaseArtifactWeapon {
    pub fn unique_name(self, name: impl Into<ArtifactWeaponName>) -> ArtifactWeaponBuilderWithName {
        ArtifactWeaponBuilder::base_weapon(self).name(name)
    }
}