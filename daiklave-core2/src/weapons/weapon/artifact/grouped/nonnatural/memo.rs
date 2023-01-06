use serde::{Deserialize, Serialize};

use super::{no_attunement::NonnaturalArtifactWeaponNoAttunementMemo, NonnaturalArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NonnaturalArtifactWeaponMemo(
    pub NonnaturalArtifactWeaponNoAttunementMemo,
    pub Option<u8>,
);

impl<'source> NonnaturalArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> NonnaturalArtifactWeapon<'source> {
        NonnaturalArtifactWeapon(self.0.as_ref(), self.1)
    }
}
