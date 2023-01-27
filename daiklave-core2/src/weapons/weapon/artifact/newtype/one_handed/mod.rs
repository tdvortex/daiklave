use std::ops::Deref;

use crate::{
    hearthstones::SlottedHearthstone, weapons::weapon::artifact::traits::ArtifactWeaponTraits,
};

mod memo;
pub use memo::OneHandedArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OneHandedArtifactWeaponView<'source>(pub(crate) ArtifactWeaponTraits<'source>);

impl<'source> Deref for OneHandedArtifactWeaponView<'source> {
    type Target = ArtifactWeaponTraits<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> OneHandedArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&self) -> OneHandedArtifactWeapon {
        OneHandedArtifactWeapon(self.0.as_memo())
    }

    pub(crate) fn hearthstone_slots_mut(
        &mut self,
    ) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        &mut self.0.hearthstone_slots
    }
}
