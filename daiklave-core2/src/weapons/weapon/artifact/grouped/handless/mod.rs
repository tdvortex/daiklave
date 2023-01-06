mod memo;
mod no_attunement;

use std::ops::Deref;

pub(crate) use memo::HandlessArtifactWeaponMemo;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

pub(crate) use no_attunement::{
    HandlessArtifactWeaponNoAttunement, HandlessArtifactWeaponNoAttunementMemo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HandlessArtifactWeapon<'source>(
    pub HandlessArtifactWeaponNoAttunement<'source>,
    pub Option<u8>,
);

impl<'source> HandlessArtifactWeapon<'source> {
    pub fn as_memo(&self) -> HandlessArtifactWeaponMemo {
        HandlessArtifactWeaponMemo(self.0.as_memo(), self.1)
    }
}

impl<'source> Deref for HandlessArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
