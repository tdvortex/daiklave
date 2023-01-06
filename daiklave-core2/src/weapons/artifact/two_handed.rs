use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::named::{NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoHandedArtifactWeapon<'source>(pub(crate) &'source NamedArtifactWeaponMemo);

impl<'source> Deref for TwoHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> TwoHandedArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> TwoHandedArtifactWeaponMemo {
        TwoHandedArtifactWeaponMemo(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> TwoHandedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> TwoHandedArtifactWeapon<'source> {
        TwoHandedArtifactWeapon(&self.0)
    }
}
