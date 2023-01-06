use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A magic, Essence-empowered weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct ArtifactWeaponId(pub UniqueId);

impl Deref for ArtifactWeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
