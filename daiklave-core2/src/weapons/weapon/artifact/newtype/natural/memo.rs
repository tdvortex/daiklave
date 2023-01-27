use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::traits::ArtifactWeaponTraitsMemo;

use super::NaturalArtifactWeaponView;

/// An artifact weapon which is part of the user's body. (This is uncommon,
/// but occurs with weapons like the Blood Lash spell).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalArtifactWeapon(pub(crate) ArtifactWeaponTraitsMemo);

impl<'source> NaturalArtifactWeapon {
    pub(crate) fn as_ref(&'source self) -> NaturalArtifactWeaponView<'source> {
        NaturalArtifactWeaponView(self.0.as_ref())
    }
}
