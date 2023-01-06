use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::named::NamedArtifactWeaponMemo;

use super::NaturalArtifactWeapon;

/// An artifact weapon which is part of the user's body. (This is uncommon,
/// but occurs with weapons like the Blood Lash spell).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> NaturalArtifactWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> NaturalArtifactWeapon<'source> {
        NaturalArtifactWeapon(self.0.as_ref())
    }
}
