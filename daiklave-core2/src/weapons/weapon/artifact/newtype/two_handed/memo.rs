use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponTraitsMemo;

/// A two-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedArtifactWeapon(pub(crate) ArtifactWeaponTraitsMemo);