use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponTraitsMemo;

use super::TwoHandedArtifactWeaponView;

/// A two-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedArtifactWeapon(pub(crate) ArtifactWeaponTraitsMemo);

impl<'source> TwoHandedArtifactWeapon {
    pub(crate) fn as_ref(&'source self) -> TwoHandedArtifactWeaponView<'source> {
        TwoHandedArtifactWeaponView(self.0.as_ref())
    }
}
