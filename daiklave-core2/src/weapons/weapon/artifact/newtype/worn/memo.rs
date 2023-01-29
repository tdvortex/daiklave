use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponTraitsMemo;
/// An artifact weapon that is worn when equipped, and does not use
/// any hands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornArtifactWeapon(pub(crate) ArtifactWeaponTraitsMemo);
