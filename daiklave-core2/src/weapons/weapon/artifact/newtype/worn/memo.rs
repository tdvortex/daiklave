use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::named::NamedArtifactWeaponMemo;

use super::WornArtifactWeapon;

/// An artifact weapon that is worn when equipped, and does not use
/// any hands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> WornArtifactWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> WornArtifactWeapon<'source> {
        WornArtifactWeapon(self.0.as_ref())
    }
}
