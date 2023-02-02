use crate::weapons::weapon::builder::base::{
    BaseArtifactWeaponBuilder, BaseArtifactWeaponBuilderWithAttack, BaseWeaponBuilder,
};

use super::{
    builder::{ArtifactWeaponBuilder, ArtifactWeaponBuilderWithName},
    ArtifactWeaponName, BaseArtifactWeapon,
};

/// A base artifact weapon which can be instantiated into a unique, named
/// artifact weapon.
pub struct AddBaseArtifactWeapon {
    pub(crate) name: String,
    pub(crate) weapon: BaseArtifactWeapon,
}

impl From<BaseArtifactWeaponBuilderWithAttack> for AddBaseArtifactWeapon {
    fn from(builder: BaseArtifactWeaponBuilderWithAttack) -> Self {
        builder.build()
    }
}

impl AddBaseArtifactWeapon {
    /// Starts constructing a new base weapon with this name.
    pub fn base_name(name: impl Into<String>) -> BaseArtifactWeaponBuilder {
        BaseWeaponBuilder::name(name).artifact()
    }

    /// Starts constructing a unique artifact weapon with this base weapon.
    pub fn unique_name(self, name: impl Into<ArtifactWeaponName>) -> ArtifactWeaponBuilderWithName {
        ArtifactWeaponBuilder::base_weapon(self).name(name)
    }
}
