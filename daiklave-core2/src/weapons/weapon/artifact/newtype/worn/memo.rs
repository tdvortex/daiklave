use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::named::NamedArtifactWeaponMemo;

use super::WornArtifactWeaponView;

/// An artifact weapon that is worn when equipped, and does not use
/// any hands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornArtifactWeapon(pub(crate) NamedArtifactWeaponMemo);

impl<'source> WornArtifactWeapon {
    pub(crate) fn as_ref(&'source self) -> WornArtifactWeaponView<'source> {
        WornArtifactWeaponView(self.0.as_ref())
    }
}
