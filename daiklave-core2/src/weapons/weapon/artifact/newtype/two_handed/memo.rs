use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::named::NamedArtifactWeaponMemo;

use super::TwoHandedArtifactWeapon;

/// A two-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> TwoHandedArtifactWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> TwoHandedArtifactWeapon<'source> {
        TwoHandedArtifactWeapon(self.0.as_ref())
    }
}
