use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::named::{NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedArtifactWeapon<'source>(pub(crate) &'source NamedArtifactWeaponMemo);

impl<'source> Deref for OneHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> OneHandedArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> OneHandedArtifactWeaponMemo {
        OneHandedArtifactWeaponMemo(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> OneHandedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedArtifactWeapon<'source> {
        OneHandedArtifactWeapon(&self.0)
    }
}
