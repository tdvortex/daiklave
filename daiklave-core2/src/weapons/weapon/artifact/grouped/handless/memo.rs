use serde::{Deserialize, Serialize};

use super::{no_attunement::HandlessArtifactWeaponNoAttunementMemo, HandlessArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HandlessArtifactWeaponMemo(
    pub HandlessArtifactWeaponNoAttunementMemo,
    pub Option<u8>,
);

impl<'source> HandlessArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> HandlessArtifactWeapon<'source> {
        HandlessArtifactWeapon(self.0.as_ref(), self.1)
    }
}
