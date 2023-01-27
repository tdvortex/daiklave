use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::traits::ArtifactWeaponTraitsMemo;

use super::OneHandedArtifactWeaponView;

/// A one-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedArtifactWeapon(pub(crate) ArtifactWeaponTraitsMemo);

impl<'source> OneHandedArtifactWeapon {
    pub(crate) fn as_ref(&'source self) -> OneHandedArtifactWeaponView<'source> {
        OneHandedArtifactWeaponView(self.0.as_ref())
    }
}
