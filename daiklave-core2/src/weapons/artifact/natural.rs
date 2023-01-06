use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::named::{NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalArtifactWeapon<'source>(pub(crate) &'source NamedArtifactWeaponMemo);

impl<'source> Deref for NaturalArtifactWeapon<'source> {
    type Target = NamedArtifactWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> NaturalArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> NaturalArtifactWeaponMemo {
        NaturalArtifactWeaponMemo(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> NaturalArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> NaturalArtifactWeapon<'source> {
        NaturalArtifactWeapon(&self.0)
    }
}
