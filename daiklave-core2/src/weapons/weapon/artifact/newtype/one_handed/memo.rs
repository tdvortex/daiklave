use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::named::NamedArtifactWeaponMemo;

use super::OneHandedArtifactWeapon;

/// A one-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> OneHandedArtifactWeaponMemo {
    /// Creates a borrowed (and copied) reference to this artifact.
    pub fn as_ref(&'source self) -> OneHandedArtifactWeapon<'source> {
        OneHandedArtifactWeapon(self.0.as_ref())
    }
}
