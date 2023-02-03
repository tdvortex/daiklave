mod memo;
mod no_attunement;

use std::ops::Deref;

pub(crate) use memo::HandlessArtifactWeaponMemo;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::inner::ArtifactWeaponInner,
};

pub(crate) use no_attunement::{
    HandlessArtifactWeaponNoAttunement, HandlessArtifactWeaponNoAttunementMemo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HandlessArtifactWeapon<'source>(
    pub HandlessArtifactWeaponNoAttunement<'source>,
    pub Option<u8>,
);

impl<'source> HandlessArtifactWeapon<'source> {
    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        self.0.hearthstone_slots_mut()
    }
}

impl<'source> Deref for HandlessArtifactWeapon<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
